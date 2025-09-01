use actix_web::{web, HttpResponse, Responder};

use qrcode::{ QrCode, render };
use crate::libraries::schemas::qr_codes::QrWifiQuery;
use crate::libraries::utils::qr_codes::build_qr_wifi_code;

#[actix_web::get("/wifi")]
pub async fn qr_wifi_handler(query: web::Query<QrWifiQuery>) -> impl Responder {
    let wifi_payload = build_qr_wifi_code(&query);

    let code = QrCode::new(wifi_payload).expect("Failed to create QR");

    let qr_svg = code
        .render::<render::svg::Color>()
        .build();

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_svg)
}
