use actix_web::web;
use crate::controllers::monitoring::health_check;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/monitoring")
            .service(health_check)
    );
}
