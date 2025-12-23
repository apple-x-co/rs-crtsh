use cli_table::Table;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Table)]
pub struct Crt {
    pub id: u64,
    pub common_name: String,
    pub entry_timestamp: String,
    pub issuer_ca_id: u64,
    pub issuer_name: String,
    pub name_value: String,
    pub not_after: String,
    pub not_before: String,
    pub result_count: u32,
    pub serial_number: String,
}
