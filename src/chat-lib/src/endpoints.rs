#![allow(clippy::missing_errors_doc)]

use crate::models::{
    Conversation, ConversationUser, Message, NewConversation, NewMessage, NewUser, User,
};
use crate::schema::{conversation_users, conversations, messages, users};
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use crate::schema;
use crate::schema::conversations::id;
use diesel::prelude::*;


pub fn create_user(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn create_message(
    conn: &mut PgConnection,
    new_message: &NewMessage,
) -> QueryResult<Message> {
    diesel::insert_into(messages::table)
        .values(new_message)
        .get_result(conn)
}

fn new_convo(conn: &mut PgConnection, title: Option<&str>) -> QueryResult<Conversation> {
    diesel::insert_into(conversations::table)
        .values(crate::models::NewConvoInternal {
            id: None,
            title}).get_result(conn)
}

pub fn create_conversation(
    conn: &mut PgConnection,
    new_conversation: &NewConversation,
) -> QueryResult<Conversation> {
    let conversation = new_convo(conn, new_conversation.title.as_deref())?;

    let sender = ConversationUser {
        user_id: new_conversation.sender,
        conversation_id: conversation.id,
    };
    let reciever = ConversationUser {
        conversation_id: conversation.id,
        user_id: new_conversation.receiver,
    };

    let _= diesel::insert_into(conversation_users::table)
        .values(sender)
        .execute(conn);
    let _ =diesel::insert_into(conversation_users::table)
        .values(reciever)
        .execute(conn);

    Ok(conversation)
}

pub fn get_user(conn: &mut PgConnection, username: &str) -> QueryResult<User> {
    match users::table.load::<User>(conn) {
        Ok(users) => users
            .into_iter()
            .find(|user| {
                user.username == username || user.id == username.parse::<i32>().unwrap_or(-1)
            })
            .ok_or(diesel::NotFound),
        Err(e) => Err(e),
    }


}
pub fn get_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(conn)
}


pub fn get_conversation(
    conn: &mut PgConnection,
    conv_id: &str
) -> Result<Vec<Message>, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use crate::schema::messages::dsl::{conversation_id, messages};

    messages
        .filter(conversation_id.eq(conv_id.parse::<i32>().unwrap_or(-1)))
        .load::<Message>(conn)
}


pub fn get_conversations(
    conn: &mut PgConnection,
    u_id: &str
) -> Result<Vec<Conversation>, diesel::result::Error> {
    let user = get_user(conn, u_id)?;
    Ok(user.get_conversations(conn))
}

pub fn get_conversation_users(
    conn: &mut PgConnection,
    conversation_id: &str
) -> Result<Vec<ConversationUser>, diesel::result::Error> {
    use crate::schema::conversations::dsl::{conversations as convs, id as conv_id};
    use crate::schema::conversation_users::dsl::{conversation_id as user_conv_id, conversation_users};

    conversation_users
        .filter(user_conv_id.eq(convs.filter(conv_id.eq(conversation_id.parse::<i32>().unwrap_or(-1))).first::<Conversation>(conn)?.id))
        .load::<ConversationUser>(conn)

}