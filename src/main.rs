mod controllers;
mod routes;
mod libraries;
mod services;
mod docs;
mod config;

use std::net::Ipv4Addr;
use actix_web::middleware::Logger;
use actix_web::{ HttpServer, App, web, Responder, HttpResponse };
use std::io::Result;

use crate::libraries::middlewares::correlation_id::{CorrelationId, CorrelationIdVariable};
use crate::routes::monitoring;
use crate::routes::qr_codes as qr_codes_router;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use crate::docs::ApiDoc;
use crate::config::Config;

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let cfg = Config::new()
        .expect("failed to load configuration from environment");

    env_logger::Builder::new()
        .parse_filters(&cfg.log_level)
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Logger::new("[%{correlation_id}xi] %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
                .add_correlation_id(),
            )
            .wrap(CorrelationId::new())
            .service(
                web::scope("/api")
                    .configure(qr_codes_router::configure)
                    .configure(monitoring::configure)
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi())
            )
            .default_service(web::route().to(not_found))
    })
    .bind((Ipv4Addr::UNSPECIFIED, cfg.port))?
    .run()
    .await
}
