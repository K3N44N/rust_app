use rocket::serde::json::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthCheck {
    health: String,
}

#[utoipa::path(
    get,
    path= "/health",
    responses(
        (status = 200, description = "Health Check", body = HealthCheck)
    )

)]

#[get("/health")]
pub fn health() -> Json<HealthCheck> {
    return Json(HealthCheck {
        health: "Hello, world!".to_string(),
    })
}