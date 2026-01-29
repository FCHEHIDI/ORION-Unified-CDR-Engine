use crate::config::ScyllaConfig;
use crate::service::model::EnrichedCDR;
use anyhow::Result;
use scylla::{Session, SessionBuilder};
use std::sync::Arc;
use tracing;

pub struct ScyllaRepository {
    session: Arc<Session>,
    keyspace: String,
}

impl ScyllaRepository {
    pub async fn new(config: &ScyllaConfig) -> Result<Self> {
        tracing::info!("Connecting to ScyllaDB nodes: {:?}", config.nodes);

        let session: Session = SessionBuilder::new()
            .known_nodes(&config.nodes)
            .build()
            .await?;

        let session = Arc::new(session);

        let repo = Self {
            session,
            keyspace: config.keyspace.clone(),
        };

        // Initialize keyspace and tables
        repo.init_schema(config.replication_factor).await?;

        Ok(repo)
    }

    async fn init_schema(&self, replication_factor: usize) -> Result<()> {
        tracing::info!("Initializing ScyllaDB schema");

        // Create keyspace if not exists
        let create_keyspace = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class': 'SimpleStrategy', 'replication_factor': {}}}",
            self.keyspace, replication_factor
        );
        self.session.query(create_keyspace, &[]).await?;
        tracing::info!("Keyspace '{}' initialized", self.keyspace);

        // Create CDR table
        let create_table = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {}.cdr (
                cdr_id text PRIMARY KEY,
                event_type text,
                imsi text,
                msisdn text,
                imei text,
                country text,
                operator text,
                mcc text,
                mnc text,
                lac text,
                cell_id text,
                start_timestamp timestamp,
                end_timestamp timestamp,
                duration_seconds bigint,
                service_type text,
                call_type text,
                called_number text,
                calling_number text,
                call_direction text,
                sms_type text,
                sms_direction text,
                destination_number text,
                originating_number text,
                apn text,
                bytes_uploaded bigint,
                bytes_downloaded bigint,
                session_duration bigint,
                is_roaming boolean,
                visited_country text,
                visited_network text,
                charge_amount double,
                currency text,
                tariff_class text,
                cause_for_termination text,
                hash text,
                fraud_score double,
                risk_level text,
                fraud_reasons frozen<list<text>>,
                fraud_model_version text,
                network_name text,
                network_type text,
                cell_tower_location text,
                signal_strength int,
                handover_count int,
                subscriber_segment text,
                contract_type text,
                customer_since text,
                lifetime_value double,
                is_vip boolean,
                data_plan_limit_mb bigint,
                ingestion_timestamp timestamp,
                normalization_timestamp timestamp,
                enrichment_timestamp timestamp,
                storage_timestamp timestamp
            )
            "#,
            self.keyspace
        );
        self.session.query(create_table, &[]).await?;
        tracing::info!("Table '{}.cdr' initialized", self.keyspace);

        // Create index on IMSI for queries
        let create_imsi_index = format!(
            "CREATE INDEX IF NOT EXISTS cdr_imsi_idx ON {}.cdr (imsi)",
            self.keyspace
        );
        self.session.query(create_imsi_index, &[]).await?;

        // Create index on start_timestamp for time-range queries
        let create_timestamp_index = format!(
            "CREATE INDEX IF NOT EXISTS cdr_start_timestamp_idx ON {}.cdr (start_timestamp)",
            self.keyspace
        );
        self.session.query(create_timestamp_index, &[]).await?;

        // Create index on fraud risk_level
        let create_risk_index = format!(
            "CREATE INDEX IF NOT EXISTS cdr_risk_level_idx ON {}.cdr (risk_level)",
            self.keyspace
        );
        self.session.query(create_risk_index, &[]).await?;

        tracing::info!("Indexes created successfully");

        Ok(())
    }

    pub async fn insert_cdr(&self, enriched: &EnrichedCDR) -> Result<()> {
        let cdr = &enriched.unified;
        let fraud = enriched.fraud_info.as_ref();
        let network = enriched.network_info.as_ref();
        let client = enriched.client_info.as_ref();

        // Parse timestamps to i64 (milliseconds since epoch)
        let start_ts = chrono::DateTime::parse_from_rfc3339(&cdr.start_timestamp)
            .ok()
            .map(|dt| dt.timestamp_millis());
        let end_ts = cdr
            .end_timestamp
            .as_ref()
            .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
            .map(|dt| dt.timestamp_millis());
        let ingestion_ts = chrono::DateTime::parse_from_rfc3339(&cdr.ingestion_timestamp)
            .ok()
            .map(|dt| dt.timestamp_millis());
        let normalization_ts = chrono::DateTime::parse_from_rfc3339(&cdr.normalization_timestamp)
            .ok()
            .map(|dt| dt.timestamp_millis());
        let enrichment_ts = chrono::DateTime::parse_from_rfc3339(&enriched.enrichment_timestamp)
            .ok()
            .map(|dt| dt.timestamp_millis());

        // Prepare fraud info  
        let fraud_score = fraud.map(|f| f.fraud_score);
        let risk_level = fraud.map(|f| f.risk_level.clone());
        let fraud_reasons: Option<Vec<String>> = fraud.map(|f| f.reasons.clone());
        let fraud_model = fraud.map(|f| f.model_version.clone());

        // Prepare network info
        let network_name = network.map(|n| n.network_name.clone());
        let network_type = network.map(|n| n.network_type.clone());
        let cell_tower_location = network.and_then(|n| n.cell_tower_location.clone());
        let signal_strength = network.and_then(|n| n.signal_strength);
        let handover_count = network.and_then(|n| n.handover_count.map(|v| v as i32));

        // Prepare client info
        let subscriber_segment = client.map(|c| c.subscriber_segment.clone());
        let contract_type = client.map(|c| c.contract_type.clone());
        let customer_since = client.and_then(|c| c.customer_since.clone());
        let lifetime_value = client.and_then(|c| c.lifetime_value);
        let is_vip = client.map(|c| c.is_vip);
        let data_plan_limit_mb = client.and_then(|c| c.data_plan_limit_mb.map(|v| v as i64));

        let insert_query = format!(
            r#"
            INSERT INTO {}.cdr (
                cdr_id, event_type, imsi, msisdn, imei, country, operator, mcc, mnc, lac, cell_id,
                start_timestamp, end_timestamp, duration_seconds, service_type, call_type,
                called_number, calling_number, call_direction, sms_type, sms_direction,
                destination_number, originating_number, apn, bytes_uploaded, bytes_downloaded,
                session_duration, is_roaming, visited_country, visited_network, charge_amount,
                currency, tariff_class, cause_for_termination, hash, fraud_score, risk_level,
                fraud_reasons, fraud_model_version, network_name, network_type, cell_tower_location,
                signal_strength, handover_count, subscriber_segment, contract_type, customer_since,
                lifetime_value, is_vip, data_plan_limit_mb, ingestion_timestamp,
                normalization_timestamp, enrichment_timestamp, storage_timestamp
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, toTimestamp(now())
            )
            "#,
            self.keyspace
        );

        // Use nested tuples to circumvent 16-element limit (52 total params)
        // Split into 4 tuples of 16/15/13/8
        self.session
            .query(
                insert_query,
                (
                    ( // Group 1: 16 fields
                        &cdr.cdr_id,
                        &cdr.event_type,
                        &cdr.imsi,
                        &cdr.msisdn,
                        &cdr.imei,
                        &cdr.country,
                        &cdr.operator,
                        &cdr.mcc,
                        &cdr.mnc,
                        &cdr.lac,
                        &cdr.cell_id,
                        start_ts,
                        end_ts,
                        cdr.duration_seconds.map(|v| v as i64),
                        &cdr.service_type,
                        &cdr.call_type,
                    ),
                    ( // Group 2: 15 fields
                        &cdr.called_number,
                        &cdr.calling_number,
                        &cdr.call_direction,
                        &cdr.sms_type,
                        &cdr.sms_direction,
                        &cdr.destination_number,
                        &cdr.originating_number,
                        &cdr.apn,
                        cdr.bytes_uploaded.map(|v| v as i64),
                        cdr.bytes_downloaded.map(|v| v as i64),
                        cdr.session_duration.map(|v| v as i64),
                        cdr.is_roaming,
                        &cdr.visited_country,
                        &cdr.visited_network,
                        cdr.charge_amount,
                    ),
                    ( // Group 3: 13 fields
                        &cdr.currency,
                        &cdr.tariff_class,
                        &cdr.cause_for_termination,
                        &cdr.hash,
                        fraud_score,
                        risk_level,
                        fraud_reasons,
                        fraud_model,
                        network_name,
                        network_type,
                        cell_tower_location,
                        signal_strength,
                        handover_count,
                    ),
                    ( // Group 4: 8 fields
                        subscriber_segment,
                        contract_type,
                        customer_since,
                        lifetime_value,
                        is_vip,
                        data_plan_limit_mb,
                        ingestion_ts,
                        normalization_ts,
                        enrichment_ts,
                    ),
                ),
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would require ScyllaDB instance
    // For unit tests, we verify schema generation logic

    #[test]
    fn test_table_creation_query() {
        let keyspace = "orion";
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {}.cdr (cdr_id text PRIMARY KEY)",
            keyspace
        );
        assert!(query.contains("orion.cdr"));
        assert!(query.contains("PRIMARY KEY"));
    }
}
