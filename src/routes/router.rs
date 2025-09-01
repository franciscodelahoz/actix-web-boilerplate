use actix_web::web;
use crate::routes::qr_codes;

pub fn router_v1() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/v1")
    .service(qr_codes::qr_router())
}
