use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;
use validator::Validate;
use crate::libs::{
    error::{AppError, AppResult},
    models::{User, CreateUser},
};
use sqlx::PgPool;

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> AppResult<Json<User>> {
    payload.validate()?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    ).bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&pool)
    .await?;
    Ok(Json(user))
}


pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound {
        resource: "User".to_string(),
        id: id.to_string()
    })?;

    Ok(Json(user))
}


pub async fn list_users(
    State(pool): State<PgPool>,
) -> AppResult<Json<Vec<User>>> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(users))
}