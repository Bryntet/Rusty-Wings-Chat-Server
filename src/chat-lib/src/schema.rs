// @generated automatically by Diesel CLI.

diesel::table! {
    conversation_users (conversation_id, user_id) {
        conversation_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    conversations (conversation_id) {
        conversation_id -> Int4,
    }
}

diesel::table! {
    messages (message_id) {
        message_id -> Int4,
        conversation_id -> Int4,
        user_id -> Int4,
        timestamp -> Timestamptz,
        message_content -> Nullable<Text>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
    }
}

diesel::joinable!(conversation_users -> conversations (conversation_id));
diesel::joinable!(conversation_users -> users (user_id));
diesel::joinable!(messages -> conversations (conversation_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversation_users,
    conversations,
    messages,
    users,
);
