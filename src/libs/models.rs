use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Serialize, FromRow)]
pub struct PostWithAuthor {
    // #[serde(flatten)]
    // pub post: Post,
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {

    #[validate(length(min = 3, max = 255))]
    pub title: String,

    #[validate(length(min = 10))]
    pub content: String,

    #[serde(default)]
    pub published: Option<bool>
}


#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Serialize, FromRow)]
pub struct CommentWithAuthor {
    pub id: Uuid,
    pub content: String,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author_name: String,
    // #[serde(flatten)]
    // pub comment: Comment,
    // pub author_name: String,

}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1))]
    pub content: String,

}