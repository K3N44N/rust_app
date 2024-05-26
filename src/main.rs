#[macro_use] extern crate rocket;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::index::index,
        routes::health::health,
        routes::users::user,
        routes::users::create_user,
        routes::users::update_user,
        routes::users::delete_user,
    ),
    components(schemas(
        routes::index::Message,
        routes::health::HealthCheck,
        routes::users::UserMessage,
        routes::users::CreateUserMessage,
        routes::users::UpdateUserMessage,
        routes::users::DeleteUserMessage,
        
    )),
)]
struct ApiDoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount(
        "/",
        SwaggerUi::new("/docs/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
    .mount("/", routes![routes::index::index])
    .mount("/", routes![routes::users::user, routes::users::create_user, routes::users::update_user, routes::users::delete_user])
    .mount("/", routes![routes::health::health])
}
