use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

/// CDR Parquet schema compatible with ScyllaDB model
pub fn cdr_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("country_code", DataType::Utf8, false),
        Field::new("timestamp", DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None), false),
        Field::new("duration_seconds", DataType::Int32, false),
        Field::new("call_type", DataType::Utf8, false),
        Field::new("msisdn_a", DataType::Utf8, false),
        Field::new("msisdn_b", DataType::Utf8, false),
        Field::new("cell_id", DataType::Utf8, true),
        Field::new("imsi", DataType::Utf8, true),
        Field::new("is_fraud", DataType::Boolean, false),
        Field::new("fraud_score", DataType::Float64, true),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_creation() {
        let schema = cdr_schema();
        assert_eq!(schema.fields().len(), 11);
        assert_eq!(schema.field(0).name(), "id");
        assert_eq!(schema.field(1).name(), "country_code");
    }
}
