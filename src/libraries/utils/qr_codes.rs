use urlencoding;
use crate::libraries::schemas::qr_codes::{
    QrEmailQuery,
    QrPhoneQuery,
    QrSMSQuery,
    QrVCalendarQuery,
    QrVCardQuery,
    QrWifiQuery
};

pub fn build_qr_wifi_code(qr_params: &QrWifiQuery) -> String {
    format!(
        "WIFI:S:{};T:{};P:{};H:{};;",
        qr_params.ssid.clone(),
        qr_params.encryption.clone(),
        qr_params.password.clone(),
        if qr_params.hidden.unwrap_or(false) { "true" } else { "" }
    )
}

pub fn build_qr_sms_code(qr_params: &QrSMSQuery) -> String {
    format!(
        "sms:{}&body={}",
        qr_params.phone,
        urlencoding::encode(&qr_params.message)
    )
}

pub fn build_qr_email_code(qr_params: &QrEmailQuery) -> String {
    format!(
        "mailto:{}?subject={}&body={}",
        qr_params.email,
        urlencoding::encode(&qr_params.subject),
        urlencoding::encode(&qr_params.body)
    )
}

pub fn build_qr_phone_code(qr_params: &QrPhoneQuery) -> String {
    format!("tel:{}", qr_params.phone)
}

pub fn build_qr_vcard_code(qr_params: &QrVCardQuery) -> String {
    format!(
        "BEGIN:VCARD\n\
         VERSION:3.0\n\
         N:{};{};;;\n\
         FN:{}\n\
         ORG:{}\n\
         TITLE:{}\n\
         TEL:{}\n\
         EMAIL:{}\n\
         URL:{}\n\
         END:VCARD",
        qr_params.last_name,
        qr_params.first_name,
        qr_params.full_name,
        qr_params.company,
        qr_params.title,
        qr_params.phone,
        qr_params.email,
        qr_params.url
    )
}

pub fn build_qr_vcalendar_code(qr_params: &QrVCalendarQuery) -> String {
    format!(
        "BEGIN:VCALENDAR\n\
         VERSION:2.0\n\
         BEGIN:VEVENT\n\
         SUMMARY:{}\n\
         DESCRIPTION:{}\n\
         DTSTART;TZID={}:{}\n\
         DTEND;TZID={}:{}\n\
         LOCATION:{}\n\
         URL:{}\n\
         END:VEVENT\n\
         END:VCALENDAR",
        qr_params.title,
        qr_params.description,
        qr_params.timezone,
        qr_params.start_datetime,
        qr_params.timezone,
        qr_params.end_datetime,
        qr_params.location,
        qr_params.url
    )
}
