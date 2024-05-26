use rocket::serde::json::Json;
use serde::Serialize;
use utoipa::ToSchema;

// LETS DO CRUD!

// ---------------------------------------------------------------------------------------
//CREATE 
// ---------------------------------------------------------------------------------------

#[derive(Serialize, ToSchema)]
pub struct CreateUserMessage {
    message: String,
}

#[utoipa::path(
    post,
    path = "/user",
    request_body = String,
    responses(
        (status = 201, description = "User Created", body = CreateUserMessage)
    )
)]
#[post("/user", data = "<user>")]
pub fn create_user(user: &str) -> Json<CreateUserMessage> {
    Json(CreateUserMessage {
        message: format!("User {} created!", user),
    })
}

//-----------------------------------------------------------------------------------------
// READ
//-----------------------------------------------------------------------------------------


#[derive(Serialize, ToSchema)]
pub struct UserMessage {
    message: String,
}

#[utoipa::path(
    get,
    path = "/user/{name}",
    params(
        ("name" = String, Path, description = "Name of the user")
    ),
    responses(
        (status = 200, description = "Greet User", body = UserMessage)
    )
)]
#[get("/user/<name>")]
pub fn user(name: &str) -> Json<UserMessage> {
    Json(UserMessage {
        message: format!("Hello, {}!", name),
    })
}

//-----------------------------------------------------------------------------------------
// UPDATE
//-----------------------------------------------------------------------------------------

#[derive(Serialize, ToSchema)]
pub struct UpdateUserMessage {
    message: String,
}

#[utoipa::path(
    patch,
    path = "/user",
    request_body = String,
    responses(
        (status = 200, description = "Update User", body = UpdateUserMessage)
    )
)]
#[patch("/user", data = "<user>")]
pub fn update_user(user: &str) -> Json<UpdateUserMessage> {
    Json(UpdateUserMessage {
        message: format!("User {} updated!", user),
    })
}

//--------------------------------------------------------------------------------
// Delete
//--------------------------------------------------------------------------------

#[derive(Serialize, ToSchema)]
pub struct DeleteUserMessage {
    message: String,
}

#[utoipa::path(
    delete,
    path = "/user",
    path = "/user/{name}",
    params(
        ("name" = String, Path, description = "Name of the user")
    ),
    responses(
        (status = 200, description = "Delete User", body = DeleteUserMessage)
    )
)]

#[delete("/user/<name>")]
pub fn delete_user(name: &str) -> Json<DeleteUserMessage> {
    Json(DeleteUserMessage {
        message: format!("User {} deleted!", name),
    })
}