mod controllers;
mod routes;
mod libraries;
mod services;
mod docs;

use std::env;
use std::net::Ipv4Addr;
use actix_web::middleware::Logger;
use actix_web::{ HttpServer, App, web, Responder, HttpResponse };
use std::io::Result;
use env_logger::Env;

use crate::routes::monitoring;
use crate::routes::qr_codes as qr_codes_router;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use crate::docs::ApiDoc;
use crate::libraries::constants::config::{DEFAULT_LOG_LEVEL, DEFAULT_PORT};

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().filter_or("LOG_LEVEL", DEFAULT_LOG_LEVEL));

    let application_port = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.into())
        .parse()
        .expect("PORT value must be a valid number");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
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
    .bind((Ipv4Addr::UNSPECIFIED, application_port))?
    .run()
    .await
}
