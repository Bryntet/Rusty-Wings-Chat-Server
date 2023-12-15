use crate::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Associations, Identifiable)]
#[diesel(table_name = schema::messages)]
#[diesel(belongs_to(Conversation))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub user_id: i32,
    pub timestamp: DateTime<Utc>,
    pub message_content: Option<String>,
}

#[derive(Identifiable, Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub username: String,
}

use crate::schema::conversations::dsl::conversations;
use schema::messages;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub message_content: String,
    pub conversation_id: i32,
    pub user_id: i32,
}

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Associations,
    Identifiable,
    Default,
    Debug,
    PartialEq,
)]
#[diesel(primary_key(conversation_id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::conversation_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConversationUser {
    pub conversation_id: i32,
    pub user_id: i32,
}

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Associations,
    Default,
    Debug,
    PartialEq,
)]
#[diesel(belongs_to(ConversationUser, foreign_key = id))]
#[diesel(table_name = schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Conversation {
    pub id: i32,
    #[diesel(column_name = name)]
    pub title: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct NewConvoInternal<'a> {
    pub id: Option<i32>,
    #[diesel(column_name = name)]
    pub title: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewConversation {
    pub sender: i32,
    pub receiver: i32,
    pub title: Option<String>,
}
