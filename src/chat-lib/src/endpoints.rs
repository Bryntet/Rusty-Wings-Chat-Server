use crate::models::{
    Conversation, ConversationUser, Message, NewConversation, NewMessage, NewUser, User,
};
use crate::schema::{conversation_users, conversations, messages, users};
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};


pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn create_message(
    conn: &mut PgConnection,
    new_message: NewMessage,
) -> QueryResult<Message> {
    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
}

fn new_convo(conn: &mut PgConnection) -> QueryResult<Conversation> {
    diesel::insert_into(conversations::table)
        .default_values().get_result(conn)
}

pub fn create_conversation(
    conn: &mut PgConnection,
    new_conversation: NewConversation,
) -> QueryResult<Conversation> {
    let conversation = new_convo(conn)?;

    let sender = ConversationUser {
        user_id: new_conversation.sender,
        conversation_id: conversation.conversation_id,
    };
    let reciever = ConversationUser {
        conversation_id: conversation.conversation_id,
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

pub fn get_user(conn: &mut PgConnection, id: &str) -> QueryResult<User> {
    match users::table.load::<User>(conn) {
        Ok(users) => users
            .into_iter()
            .find(|user| user.user_id == id.parse::<i32>().unwrap_or(-1))
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
    use crate::schema::messages::dsl::*;


    messages
        .filter(conversation_id.eq(conv_id.parse::<i32>().unwrap_or(-1)))
        .load::<Message>(conn)
}

pub fn get_conversations(
    conn: &mut PgConnection,
    u_id: &str
) -> Result<Vec<Conversation>, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use crate::schema::conversation_users::dsl::*;
    Ok(conversation_users
        .filter(user_id.eq(u_id.parse::<i32>().unwrap_or(-1)))
        .get_results::<ConversationUser>(conn)?.iter().map(|c_u| Conversation{conversation_id: c_u.conversation_id}).collect())
}