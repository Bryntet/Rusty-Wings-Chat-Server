-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL
    -- other fields can be added here
);

CREATE TABLE conversations (
    id SERIAL NOT NULL PRIMARY KEY UNIQUE
);


CREATE TABLE conversation_users (
    conversation_id INT NOT NULL,
    user_id INT NOT NULL,
    PRIMARY KEY (conversation_id, user_id),
    FOREIGN KEY (conversation_id) REFERENCES conversations(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);



