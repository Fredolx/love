use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Interface {
    pub name: String,
    pub ip: String,
    pub mac: String
}