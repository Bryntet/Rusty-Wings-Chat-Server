mod handler;
use rusty_wings_chat_lib::*;


use axum::{
    routing::{get, post},
    Router,
};

use std::sync::Arc;


use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let app = make_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
fn make_router() -> Router {
    use handler::*;

    let pool = Arc::new(Mutex::new(establish_connection()));

    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route(
            "/create-user",
            post({
                let pool = Arc::clone(&pool);
                move |body| create_user(body, pool)
            }),
        )
        .route(
            "/create-message",
            post({
                let pool = Arc::clone(&pool);
                move |body| create_message(body, pool)
            }),
        )
        .route(
            "/user/:id",
            get({
                let pool = Arc::clone(&pool);
                move |path| get_user(path, Arc::clone(&pool))
            }),
        )
        .route(
            "/users/",
            get({
                let pool = Arc::clone(&pool);
                move || get_users(pool)
            }),
        )
        .route(
            "/create-conversation",
            post({
                let pool = Arc::clone(&pool);
                move |body| create_conversation(body, pool)
            }),
        )
        .route(
            "/conversation/:conversation_id",
            get({
                let pool = Arc::clone(&pool);
                move |path| get_conversation(path, Arc::clone(&pool))
            }),
        )
        .route(
            "/conversations/:user_id",
            get({
                let pool = Arc::clone(&pool);
                move |path| get_conversations(path, Arc::clone(&pool))
            }),
        )
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
