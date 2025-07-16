use crate::models::{CreateUserRequest, FingerprintResponse, UpdateUserRequest, User};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::users::list_users,
        crate::api::handlers::users::get_user,
        crate::api::handlers::users::create_user,
        crate::api::handlers::users::update_user,
        crate::api::handlers::users::delete_user,
        crate::api::handlers::fingerprint::get_fingerprint,
        crate::api::handlers::fingerprint::get_image,
        crate::api::handlers::fingerprint::health_check,
    ),
    components(
        schemas(User, CreateUserRequest, UpdateUserRequest, FingerprintResponse)
    ),
    tags(
        (name = "users", description = "Operações sobre users"),
        (name = "fingerprint", description = "Operações sobre fingerprint")
    ),
    servers(
        (url = "/api/v1", description = "API base path")
    )
)]
pub struct ApiDoc;
