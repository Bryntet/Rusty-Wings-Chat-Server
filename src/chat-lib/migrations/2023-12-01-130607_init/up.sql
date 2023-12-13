-- Your SQL goes here
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL
    -- other fields can be added here
);

CREATE TABLE conversations (
    conversation_id SERIAL NOT NULL PRIMARY KEY UNIQUE
);


CREATE TABLE conversation_users (
    conversation_id INT NOT NULL,
    user_id INT NOT NULL,
    PRIMARY KEY (conversation_id, user_id),
    FOREIGN KEY (conversation_id) REFERENCES conversations(conversation_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);



