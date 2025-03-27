use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QrWifiQuery {
    pub ssid: String,
    pub encryption: String,
    pub password: String,
    pub hidden: Option<bool>,
}
