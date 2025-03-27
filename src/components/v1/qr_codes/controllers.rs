use actix_web::{web, HttpResponse, Responder};

use super::services;

use super::schemas::QrWifiQuery;

#[actix_web::get("/wifi")]
async fn qr_wifi_handler(query: web::Query<QrWifiQuery>) -> impl Responder {
    let qr_result = services::qr_wifi_handler(query.into_inner()).await;

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(qr_result)
}

pub fn qr_router() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/qr")
        .service(qr_wifi_handler)
}
