use actix_web::{web, HttpResponse, Responder};

use crate::libraries::schemas::qr_codes::{
    QrEmailQuery,
    QrPhoneQuery,
    QrSMSQuery,
    QrVCalendarQuery,
    QrVCardQuery,
    QrWifiQuery
};
use crate::services::qr_codes::{
    generate_qr_wifi_code,
    generate_qr_sms_code,
    generate_qr_email_code,
    generate_qr_phone_code,
    generate_qr_vcard_code,
    generate_qr_vcalendar_code
};

#[actix_web::get("/wifi")]
pub async fn qr_wifi_handler(query: web::Query<QrWifiQuery>) -> impl Responder {
    let qr_svg = generate_qr_wifi_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[actix_web::get("/sms")]
pub async fn qr_sms_handler(query: web::Query<QrSMSQuery>) -> impl Responder {
    let qr_svg = generate_qr_sms_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[actix_web::get("/email")]
pub async fn qr_email_handler(query: web::Query<QrEmailQuery>) -> impl Responder {
    let qr_svg = generate_qr_email_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[actix_web::get("/phone")]
pub async fn qr_phone_handler(query: web::Query<QrPhoneQuery>) -> impl Responder {
    let qr_svg = generate_qr_phone_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[actix_web::get("/vcard")]
pub async fn qr_vcard_handler(query: web::Query<QrVCardQuery>) -> impl Responder {
    let qr_svg = generate_qr_vcard_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[actix_web::get("/vcalendar")]
pub async fn qr_vcalendar_handler(query: web::Query<QrVCalendarQuery>) -> impl Responder {
    let qr_svg = generate_qr_vcalendar_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}
