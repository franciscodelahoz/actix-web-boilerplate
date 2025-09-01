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

#[utoipa::path(
    get,
    path = "/wifi",
    context_path = "/api/v1/qr",
    description = "Generate a WiFi QR code. The generated QR code can be scanned by mobile devices to automatically connect to a WiFi network using the provided SSID and password.",
    tag = "QR Codes",
    params(QrWifiQuery),
    responses(
        (status = 200, description = "QR code in SVG format for WiFi network", body = String)
    )
)]
#[actix_web::get("/wifi")]
pub async fn qr_wifi_handler(query: web::Query<QrWifiQuery>) -> impl Responder {
    let qr_svg = generate_qr_wifi_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[utoipa::path(
    get,
    path = "/sms",
    context_path = "/api/v1/qr",
    description = "Generate an SMS QR code. Scanning this QR code allows users to send a predefined SMS message to a specific phone number.",
    tag = "QR Codes",
    params(QrSMSQuery),
    responses(
        (status = 200, description = "QR code in SVG format for sending SMS", body = String)
    )
)]
#[actix_web::get("/sms")]
pub async fn qr_sms_handler(query: web::Query<QrSMSQuery>) -> impl Responder {
    let qr_svg = generate_qr_sms_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[utoipa::path(
    get,
    path = "/email",
    context_path = "/api/v1/qr",
    description = "Generate an Email QR code. Scanning this QR opens the default email client with predefined recipient, subject, and body.",
    tag = "QR Codes",
    params(QrEmailQuery),
    responses(
        (status = 200, description = "QR code in SVG format for sending email", body = String)
    )
)]
#[actix_web::get("/email")]
pub async fn qr_email_handler(query: web::Query<QrEmailQuery>) -> impl Responder {
    let qr_svg = generate_qr_email_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[utoipa::path(
    get,
    path = "/phone",
    context_path = "/api/v1/qr",
    description = "Generate a Phone QR code. Scanning this QR code allows users to call a specific phone number directly.",
    tag = "QR Codes",
    params(QrPhoneQuery),
    responses(
        (status = 200, description = "QR code in SVG format for phone calls", body = String)
    )
)]
#[actix_web::get("/phone")]
pub async fn qr_phone_handler(query: web::Query<QrPhoneQuery>) -> impl Responder {
    let qr_svg = generate_qr_phone_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[utoipa::path(
    get,
    path = "/vcard",
    context_path = "/api/v1/qr",
    description = "Generate a VCard QR code. Scanning this QR code allows users to save contact information directly to their address book.",
    tag = "QR Codes",
    params(QrVCardQuery),
    responses(
        (status = 200, description = "QR code in SVG format for VCard contact", body = String)
    )
)]
#[actix_web::get("/vcard")]
pub async fn qr_vcard_handler(query: web::Query<QrVCardQuery>) -> impl Responder {
    let qr_svg = generate_qr_vcard_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}

#[utoipa::path(
    get,
    path = "/vcalendar",
    context_path = "/api/v1/qr",
    description = "Generate a VCalendar QR code. Scanning this QR code allows users to add calendar events with title, description, date, and time.",
    tag = "QR Codes",
    params(QrVCalendarQuery),
    responses(
        (status = 200, description = "QR VCalendar SVG", body = String)
    )
)]
#[actix_web::get("/vcalendar")]
pub async fn qr_vcalendar_handler(query: web::Query<QrVCalendarQuery>) -> impl Responder {
    let qr_svg = generate_qr_vcalendar_code(&query);

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}
