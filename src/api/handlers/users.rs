use crate::models::{CreateUserRequest, UpdateUserRequest, UserResponse, UsersListResponse};
use crate::services::user::UserService;
use axum::{Extension, Json, http::StatusCode};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = UsersListResponse)
    )
)]
pub async fn list_users(
    Extension(service): Extension<UserService>,
) -> Result<Json<UsersListResponse>, StatusCode> {
    let users = service
        .list_users()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let resp = UsersListResponse {
        total: users.len() as i64,
        users: users.into_iter().map(UserResponse::from).collect(),
    };
    Ok(Json(resp))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
pub async fn get_user(
    Extension(service): Extension<UserService>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = service
        .get_user(id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(UserResponse::from(user)))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn create_user(
    Extension(service): Extension<UserService>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let user = service
        .create_user(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
pub async fn update_user(
    Extension(service): Extension<UserService>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = service
        .update_user(id, req)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(UserResponse::from(user)))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
pub async fn delete_user(
    Extension(service): Extension<UserService>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> StatusCode {
    match service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    }
}
