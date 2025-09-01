use actix_web::web;
use crate::controllers::qr_codes::{
    qr_email_handler,
    qr_phone_handler,
    qr_sms_handler,
    qr_vcalendar_handler,
    qr_vcard_handler,
    qr_wifi_handler
};

pub fn qr_v1_router() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/v1/qr")
        .service(qr_wifi_handler)
        .service(qr_sms_handler)
        .service(qr_email_handler)
        .service(qr_phone_handler)
        .service(qr_vcard_handler)
        .service(qr_vcalendar_handler)
}
