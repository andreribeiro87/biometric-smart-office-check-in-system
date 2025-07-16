use crate::api::handlers::users::*;
use crate::config::DbPool;
use crate::models::fingerprint::*;
use crate::repository::user::*;
use crate::services::user::UserService;
use axum::{Extension, Router, routing::get};
use rumqttc::AsyncClient;

pub fn create_user_router(pool: DbPool) -> Router {
    let repo = UserRepo::new(pool.clone());
    let service = UserService::new(repo);

    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .layer(Extension(service))
}

pub fn create_mqtt_router(state: AppState) -> Router<AppState> {
    Router::new()
        // Create Axum Router
        .route(
            "/fingerprint",
            get(crate::api::handlers::fingerprint::get_fingerprint),
        )
        .route("/image", get(crate::api::handlers::fingerprint::get_image))
        .route(
            "/health",
            get(crate::api::handlers::fingerprint::health_check),
        )
        .layer(Extension(state))
}
