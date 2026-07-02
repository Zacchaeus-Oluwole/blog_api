mod libs;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::libs::{db, handlers::{
        comments::{
            create_comment, list_comments
        }, posts::{
            create_post, get_post, list_posts
        }, users::{
            create_user, get_user, list_users
        }
    },
};

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "blog_api=debug".into())
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv::dotenv().ok();
    init_tracing();

    let pool = db::create_pool().await?;

    let app = Router::new()
        .route("/users", post(create_user).get(list_users))
        .route("/users/{id}", get(get_user))
        .route("/posts", get(list_posts))
        .route("/users/{user_id}/posts", post(create_post))
        .route("/posts/{id}", get(get_post))
        .route("/posts/{post_id}/comments/{user_id}", post(create_comment))
        .route("/posts/{post_id}/comments", get(list_comments))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())

    
}
