use crate::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub message_id: i32,
    pub conversation_id: i32,
    pub user_id: i32,
    pub timestamp: DateTime<Utc>,
    pub message_content: Option<String>,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
}

use crate::schema::users;
#[derive(Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
}

use crate::schema::messages;
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub message_content: String,
    pub conversation_id: i32,
    pub user_id: i32,
}

use crate::schema::conversation_users;
#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Default, Debug)]
#[diesel(table_name = conversation_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConversationUser {
    pub conversation_id: i32,
    pub user_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Default, Debug)]
#[diesel(table_name = crate::schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Conversation {
    pub conversation_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewConvoInternal {
    pub conversation_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct NewConversation {
    pub sender: i32,
    pub receiver: i32,
}