use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};

use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    ValidationError(validator::ValidationErrors),
    NotFound {resource: String, id: String},
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(err) => {
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({
                        "error": {
                            "code": "DATABASE_ERROR",
                            "message": "A database error occurred"
                        }
                    }),
                )
            }

            AppError::ValidationError(err) => {
                // tracing::error!("Database error: {}", err);
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    json!({
                        "error": {
                            "code": "VALIDATION_ERROR",
                            "message": "Validation failed",
                            "details": err.field_errors()
                        }
                    }),
                )
            }

            AppError::NotFound { resource, id } => {
                (
                    StatusCode::NOT_FOUND,
                    json!({
                        "error": {
                            "code": "NOT_FOUND",
                            "message": format!("{} with id {} not found", resource, id)
                        }
                    })
                )
            }
        };


        (status, Json(error_message)).into_response()

        


    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;