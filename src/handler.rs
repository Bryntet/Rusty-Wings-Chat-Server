use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use diesel::PgConnection;
use rusty_wings_chat_lib::models::*;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;
macro_rules! create_post_handler {
    ($name:ident, $request:ty) => {
        pub(crate) async fn $name(
            Json(data): Json<$request>,
            conn: Arc<Mutex<PgConnection>>,
        ) -> impl IntoResponse {
            let mut conn = conn.lock().await;
            match rusty_wings_chat_lib::endpoints::$name(conn.deref_mut(), data) {
                Ok(j) => Ok(Json(j)),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
    };
}

/*pub(crate) async fn create_user(Json(data): Json<NewUser>, conn: Arc<Mutex<PgConnection>>) -> impl IntoResponse {
    let mut conn = conn.lock().await;
    match rusty_wings_chat_lib::endpoints::create_user(conn.deref_mut(), data) {
        Ok(j) => Ok(Json(j)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}*/

macro_rules! create_get_handler {
    ($name:ident) => {
        pub(crate) async fn $name(
            Path(path): Path<String>,
            conn: Arc<Mutex<PgConnection>>,
        ) -> impl IntoResponse {
            match rusty_wings_chat_lib::endpoints::$name((conn.lock().await).deref_mut(), path.as_str()) {
                Ok(j) => Ok(Json(j)),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
    };
}
pub(crate) async fn get_users(
    conn: Arc<Mutex<PgConnection>>,
) -> impl IntoResponse {
    match rusty_wings_chat_lib::endpoints::get_users((conn.lock().await).deref_mut()) {
        Ok(j) => Ok(Json(j)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
create_post_handler!(create_user, NewUser);
create_post_handler!(create_message, NewMessage);
create_post_handler!(create_conversation, NewConversation);

create_get_handler!(get_user);
create_get_handler!(get_conversation);
create_get_handler!(get_conversations);
