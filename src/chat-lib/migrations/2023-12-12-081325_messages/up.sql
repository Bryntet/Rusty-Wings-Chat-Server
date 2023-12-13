-- Your SQL goes here
CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,
    conversation_id SERIAL NOT NULL,
    user_id INT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    message_content TEXT,
    FOREIGN KEY (conversation_id) REFERENCES conversations(conversation_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);