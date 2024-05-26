use rocket::serde::json::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Message {
    message: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Index", body = Message)
    )
)]
#[get("/")]
pub fn index() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".to_string(),
    })
}
