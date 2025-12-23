use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Crt {
    pub id: u64,
    pub common_name: String,
    pub entry_timestamp: Option<String>, // NOTE: null が入ることがある
    pub issuer_ca_id: i64, // NOTE: -1 が入ることがある
    pub issuer_name: String,
    pub name_value: String,
    pub not_after: String,
    pub not_before: String,
    pub result_count: u32,
    pub serial_number: String,
}
