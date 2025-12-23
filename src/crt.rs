use cli_table::Table;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Table)]
pub struct Crt {
    #[table(title = "crt.sh ID")]
    pub id: u64,
    #[table(title = "Matching Identities")]
    pub common_name: String,
    #[table(title = "Logged At")]
    pub entry_timestamp: String,
    #[table(title = "Issuer CA ID")]
    pub issuer_ca_id: u64,
    #[table(title = "Issuer Name")]
    pub issuer_name: String,
    // #[table(title = "SAN")]
    // pub name_value: String,
    #[table(title = "Not Before")]
    pub not_after: String,
    #[table(title = "Not After")]
    pub not_before: String,
    pub result_count: u32,
    #[table(title = "Serial Number")]
    pub serial_number: String,
}
