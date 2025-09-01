use actix_web::{HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/health",
    context_path = "/api/monitoring",
    description = "Check the health status of the API.",
    tag = "Monitoring",
    responses(
        (status = 200, description = "API is healthy", body = String)
    )
)]
#[actix_web::get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
