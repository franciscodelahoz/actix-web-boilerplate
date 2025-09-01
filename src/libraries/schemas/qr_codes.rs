use serde::{Serialize, Deserialize};
use utoipa::{ToSchema, IntoParams};

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct QrWifiQuery {
    pub ssid: String,
    pub encryption: String,
    pub password: String,
    pub hidden: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct QrSMSQuery {
    pub phone: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct QrEmailQuery {
    pub email: String,
    pub subject: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct QrPhoneQuery {
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
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

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct QrVCalendarQuery {
    pub title: String,
    pub description: String,
    pub start_datetime: String,
    pub end_datetime: String,
    pub timezone: String,
    pub location: String,
    pub url: String,
}
