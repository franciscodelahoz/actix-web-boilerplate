use actix_web::web;
use crate::controllers::qr_codes::qr_wifi_handler;

pub fn qr_router() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/qr")
        .service(qr_wifi_handler)
}
