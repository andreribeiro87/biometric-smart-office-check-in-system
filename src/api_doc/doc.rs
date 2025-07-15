use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::list_users,
        crate::api::handlers::get_user,
        crate::api::handlers::create_user,
        crate::api::handlers::update_user,
        crate::api::handlers::delete_user,
    ),
    components(
        schemas(User, CreateUserRequest, UpdateUserRequest)
    ),
    tags(
        (name = "users", description = "Operações sobre users")
    ),
    servers(
        (url = "/api/v1", description = "API base path")
    )
)]
pub struct ApiDoc;
