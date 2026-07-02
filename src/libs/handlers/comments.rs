use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;
use validator::Validate;
use crate::libs::{
    error::{AppError, AppResult}, models::{Comment, CommentWithAuthor, CreateComment},
};
use sqlx::PgPool;

pub async fn create_comment(
    State(pool): State<PgPool>,
    Path((post_id,user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateComment>,
) -> AppResult<Json<Comment>> {
    payload.validate()?;

    // Verify post exists

    let post_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)"
    )
    .bind(post_id)
    .fetch_one(&pool)
    .await?;

    if !post_exists {
        return Err(AppError::NotFound { 
            resource:"Post".to_string(), 
            id: post_id.to_string(),
        });
    }

    let comment =  sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (content, post_id, user_id) 
        VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.content)
    .bind(post_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(comment))
}


pub async fn list_comments(
    State(pool): State<PgPool>,
    Path(post_id): Path<Uuid>,
) -> AppResult<Json<Vec<CommentWithAuthor>>> {
    let comments = sqlx::query_as::<_, CommentWithAuthor>(
        "SELECT c.*, u.name as author_name 
        FROM comments c 
        JOIN users u ON c.user_id = u.id 
        WHERE c.post_id = $1
        ORDER BY c.created_at DESC"
    )
    .bind(post_id)
    .fetch_all(&pool)
    .await?;

    // let comment_with_author: Vec<CommentWithAuthor> = comments.into_iter().map(|(comment, author_name)| {
    //     CommentWithAuthor {
    //         comment,
    //         author_name,
    //     }
    // }).collect();

    Ok(Json(comments))
}