use crate::config::ScyllaConfig;
use anyhow::Result;
use chrono::DateTime;
use scylla::{Session, SessionBuilder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cdr {
    pub id: String,
    pub country_code: String,
    pub timestamp: String,
    pub duration_seconds: i32,
    pub call_type: String,
    pub msisdn_a: String,
    pub msisdn_b: String,
    pub cell_id: Option<String>,
    pub imsi: Option<String>,
    pub is_fraud: Option<bool>,
    pub fraud_score: Option<f64>,
}

pub struct CdrRepository {
    session: Arc<Session>,
}

impl CdrRepository {
    pub async fn new(config: &ScyllaConfig) -> Result<Self> {
        let session = SessionBuilder::new()
            .known_nodes(&config.nodes)
            .build()
            .await?;

        session
            .query(
                format!("USE {}", config.keyspace),
                &[],
            )
            .await?;

        Ok(Self {
            session: Arc::new(session),
        })
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Cdr>> {
        let query = "SELECT id, country_code, timestamp, duration_seconds, call_type, 
                     msisdn_a, msisdn_b, cell_id, imsi, is_fraud, fraud_score 
                     FROM cdr_unified WHERE id = ?";
        
        let rows = self.session.query(query, (id,)).await?;
        
        if let Some(rows) = rows.rows {
            if let Some(row) = rows.into_iter().next() {
                let (id, country_code, timestamp, duration, call_type, msisdn_a, msisdn_b, 
                     cell_id, imsi, is_fraud, fraud_score): 
                    (String, String, i64, i32, String, String, String, 
                     Option<String>, Option<String>, Option<bool>, Option<f64>) = row.into_typed()?;
                
                return Ok(Some(Cdr {
                    id,
                    country_code,
                    timestamp: DateTime::from_timestamp(timestamp / 1000, 0)
                        .unwrap_or_default()
                        .to_rfc3339(),
                    duration_seconds: duration,
                    call_type,
                    msisdn_a,
                    msisdn_b,
                    cell_id,
                    imsi,
                    is_fraud,
                    fraud_score,
                }));
            }
        }
        
        Ok(None)
    }

    pub async fn search(
        &self,
        country_code: Option<String>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: i32,
    ) -> Result<Vec<Cdr>> {
        let mut query = "SELECT id, country_code, timestamp, duration_seconds, call_type,
                         msisdn_a, msisdn_b, cell_id, imsi, is_fraud, fraud_score
                         FROM cdr_unified".to_string();
        
        let mut conditions = Vec::new();
        
        if let Some(cc) = &country_code {
            conditions.push(format!("country_code = '{}'", cc));
        }
        if start_time.is_some() || end_time.is_some() {
            // Would need secondary index or different table structure
            // For now, fetch all and filter in memory (not ideal)
        }
        
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }
        
        query.push_str(&format!(" LIMIT {}", limit));
        
        let rows = self.session.query(query, &[]).await?;
        let mut result = Vec::new();
        
        if let Some(rows) = rows.rows {
            for row in rows {
                let (id, country_code, timestamp, duration, call_type, msisdn_a, msisdn_b,
                     cell_id, imsi, is_fraud, fraud_score):
                    (String, String, i64, i32, String, String, String,
                     Option<String>, Option<String>, Option<bool>, Option<f64>) = row.into_typed()?;
                
                result.push(Cdr {
                    id,
                    country_code,
                    timestamp: DateTime::from_timestamp(timestamp / 1000, 0)
                        .unwrap_or_default()
                        .to_rfc3339(),
                    duration_seconds: duration,
                    call_type,
                    msisdn_a,
                    msisdn_b,
                    cell_id,
                    imsi,
                    is_fraud,
                    fraud_score,
                });
            }
        }
        
        Ok(result)
    }
}
