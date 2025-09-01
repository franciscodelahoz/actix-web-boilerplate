use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QrWifiQuery {
    pub ssid: String,
    pub encryption: String,
    pub password: String,
    pub hidden: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QrSMSQuery {
    pub phone: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QrEmailQuery {
    pub email: String,
    pub subject: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QrPhoneQuery {
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QrVCardQuery {
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub company: String,
    pub title: String,
    pub phone: String,
    pub email: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QrVCalendarQuery {
    pub title: String,
    pub description: String,
    pub start_datetime: String,
    pub end_datetime: String,
    pub timezone: String,
    pub location: String,
    pub url: String,
}
