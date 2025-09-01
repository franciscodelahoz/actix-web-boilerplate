use qrcode::{ QrCode, render };
use crate::libraries::schemas::qr_codes::{
    QrEmailQuery,
    QrPhoneQuery,
    QrSMSQuery,
    QrVCalendarQuery,
    QrVCardQuery,
    QrWifiQuery
};
use crate::libraries::utils::qr_codes::{
    build_qr_email_code,
    build_qr_phone_code,
    build_qr_sms_code,
    build_qr_vcalendar_code,
    build_qr_vcard_code,
    build_qr_wifi_code
};

pub fn generate_qr_wifi_code(qr_params: &QrWifiQuery) -> String {
    let wifi_payload = build_qr_wifi_code(qr_params);

    let code = QrCode::new(wifi_payload).expect("Failed to create QR");
    code.render::<render::svg::Color>().build()
}

pub fn generate_qr_sms_code(qr_params: &QrSMSQuery) -> String {
    let sms_payload = build_qr_sms_code(qr_params);

    let code = QrCode::new(sms_payload).expect("Failed to create QR");
    code.render::<render::svg::Color>().build()
}

pub fn generate_qr_email_code(qr_params: &QrEmailQuery) -> String {
    let email_payload = build_qr_email_code(qr_params);

    let code = QrCode::new(email_payload).expect("Failed to create QR");
    code.render::<render::svg::Color>().build()
}

pub fn generate_qr_phone_code(qr_params: &QrPhoneQuery) -> String {
    let phone_payload = build_qr_phone_code(qr_params);

    let code = QrCode::new(phone_payload).expect("Failed to create QR");
    code.render::<render::svg::Color>().build()
}

pub fn generate_qr_vcard_code(qr_params: &QrVCardQuery) -> String {
    let vcard_payload = build_qr_vcard_code(qr_params);

    let code = QrCode::new(vcard_payload).expect("Failed to create QR");
    code.render::<render::svg::Color>().build()
}

pub fn generate_qr_vcalendar_code(qr_params: &QrVCalendarQuery) -> String {
    let vcalendar_payload = build_qr_vcalendar_code(qr_params);

    let code = QrCode::new(vcalendar_payload).expect("Failed to create QR");
    code.render::<render::svg::Color>().build()
}
