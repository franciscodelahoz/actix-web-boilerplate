mod components;
mod libraries;

use actix_web::{ HttpServer, App, web, Responder, HttpResponse };
use std::io::Result;

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(components::v1::router::router_v1())
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
