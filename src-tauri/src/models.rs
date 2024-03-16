use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Interface {
    pub name: String,
    pub ip: String,
    pub mac: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LanClient {
    pub hostname: Option<String>,
    pub ip: String,
    pub mac: String,
    pub vendor: Option<String>
}