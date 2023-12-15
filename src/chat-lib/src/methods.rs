use crate::models::{Conversation, ConversationUser, User};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::PgConnection;

impl User {
    fn get_conversation_users(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<ConversationUser>, DieselError> {
        ConversationUser::belonging_to(self).load::<ConversationUser>(conn)
    }

    pub fn get_conversations(&self, conn: &mut PgConnection) -> Vec<Conversation> {
        self.get_conversation_users(conn)
            .unwrap_or_default()
            .iter()
            .filter_map(|cu| cu.get_conversation(conn).ok())
            .flatten()
            .collect::<Vec<Conversation>>()
    }
}

impl ConversationUser {
    pub fn get_conversation(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Conversation>, DieselError> {
        Conversation::belonging_to(self).load::<Conversation>(conn)
    }
}
