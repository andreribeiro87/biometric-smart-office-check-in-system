use crate::api::handlers::users::*;
use crate::config::DbPool;
use crate::repository::user::*;
use crate::services::user::UserService;
use axum::{Extension, Router, routing::get};

pub fn create_router(pool: DbPool) -> Router {
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
