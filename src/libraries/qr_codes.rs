use urlencoding;

pub struct QRWifiCodeParams {
    pub ssid: String,
    pub encryption: String,
    pub password: String,
    pub hidden: bool,
}

pub struct QrSMSParams {
    pub phone: String,
    pub message: String,
}

pub struct QrEmailParams {
    pub email: String,
    pub subject: String,
    pub body: String,
}

pub struct QrPhoneParams {
    pub phone: String,
}

pub struct QrVCardParams {
    pub last_name: String,
    pub first_name: String,
    pub full_name: String,
    pub company: String,
    pub title: String,
    pub phone: String,
    pub email: String,
    pub url: String,
}

pub struct QrVCalendarParams {
    pub title: String,
    pub description: String,
    pub timezone: String,
    pub start_date: String,
    pub end_date: String,
    pub location: String,
    pub url: String,
}

pub fn build_qr_wifi_code(qr_params: &QRWifiCodeParams) -> String {
    format!(
        "WIFI:S:{};T:{};P:{};H:{};;",
        qr_params.ssid,
        qr_params.encryption,
        qr_params.password,
        if qr_params.hidden { "true" } else { "" }
    )
}

pub fn build_qr_sms_code(qr_params: &QrSMSParams) -> String {
    format!(
        "sms:{}&body={}",
        qr_params.phone,
        urlencoding::encode(&qr_params.message)
    )
}

pub fn build_qr_email_code(qr_params: &QrEmailParams) -> String {
    format!(
        "mailto:{}?subject={}&body={}",
        qr_params.email,
        urlencoding::encode(&qr_params.subject),
        urlencoding::encode(&qr_params.body)
    )
}

pub fn build_qr_phone_code(qr_params: &QrPhoneParams) -> String {
    format!("tel:{}", qr_params.phone)
}

pub fn build_qr_vcard_code(qr_params: &QrVCardParams) -> String {
    format!(
        "BEGIN:VCARD\nVERSION:3.0\nN:{};{};;;\nFN:{}\nORG:{}\nTITLE:{}\nTEL:{}\nEMAIL:{}\nURL:{}\nEND:VCARD",
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

pub fn build_qr_vcalendar_code(qr_params: &QrVCalendarParams) -> String {
    format!(
        "BEGIN:VCALENDAR\nVERSION:2.0\nBEGIN:VEVENT\nSUMMARY:{}\nDESCRIPTION:{}\nDTSTART;TZID={}:{}\nDTEND;TZID={}:{}\nLOCATION:{}\nURL:{}\nEND:VEVENT\nEND:VCALENDAR",
        qr_params.title,
        qr_params.description,
        qr_params.timezone,
        qr_params.start_date,
        qr_params.timezone,
        qr_params.end_date,
        qr_params.location,
        qr_params.url
    )
}
