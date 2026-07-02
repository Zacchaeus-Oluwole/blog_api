use axum::{
    Json, extract::{Path, Query, State},
};
use uuid::Uuid;
use serde::Deserialize;
use validator::Validate;
use crate::libs::{
    error::{AppError, AppResult},
    models::{Post, CreatePost, PostWithAuthor},
};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct Pagenation{
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 10 }


pub async fn create_post(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<CreatePost>,
) -> AppResult<Json<Post>> {
    payload.validate()?;

    // Verify user exists
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    if !user_exists {
        return Err(AppError::NotFound {
            resource: "User".to_string(),
            id: user_id.to_string()
        });
    }

    let published = payload.published.unwrap_or(false);

    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content, user_id, published) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(user_id)
    .bind(published)
    .fetch_one(&pool)
    .await?;
    Ok(Json(post))
}


pub async fn get_post(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<PostWithAuthor>> {
    let result = sqlx::query_as::<_, PostWithAuthor>(
        "SELECT p.*, u.name as author_name 
        FROM posts p 
        JOIN users u ON p.user_id = u.id
        WHERE p.id = $1"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound { 
        resource: "Post".to_string(), 
        id: id.to_string()
    })?;

    // let (post, author_name) = result;

    // Ok(Json(PostWithAuthor{
    //     post,
    //     author_name
    // }))

    Ok(Json(result))
}

pub async fn list_posts(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagenation>,
) -> AppResult<Json<Vec<PostWithAuthor>>> {
    let offset = (pagination.page - 1) * pagination.limit;

    let posts = sqlx::query_as::<_, PostWithAuthor>(
        "SELECT p.*, u.name as author_name
        FROM posts p
        JOIN users u ON p.user_id = u.id
        WHERE p.published = true
        ORDER BY p.created_at DESC
        LIMIT $1 OFFSET $2"
    )
    .bind(pagination.limit as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await?;


    // let posts_with_author: Vec<PostWithAuthor> = posts
    //     .into_iter()
    //     .map(|(post, author_name)| PostWithAuthor {
    //         post,
    //         author_name
    //     })
    //     .collect();

    Ok(Json(posts))
}