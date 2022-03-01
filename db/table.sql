CREATE TABLE local_db.users(
    user_id VARCHAR(26) PRIMARY KEY NOT NULL,
    user_name VARCHAR(50) NOT NULL,
    e_mail VARCHAR(254) NOT NULL,
    password_hash VARCHAR(128) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
CREATE TABLE local_db.follows(
    follow_user_id VARCHAR(26) NOT NULL REFERENCES users(user_id),
    follower_user_id VARCHAR(26) NOT NULL REFERENCES users(user_id),
    PRIMARY KEY (follow_user_id, follower_user_id)
);
CREATE TABLE local_db.messages(
    message_id VARCHAR(26) NOT NULL PRIMARY KEY,
    user_id VARCHAR(26) NOT NULL REFERENCES users(user_id),
    message_text TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE local_db.likes(
    user_id VARCHAR(26) NOT NULL REFERENCES users(user_id),
    message_id VARCHAR(26) NOT NULL REFERENCES messages(message_id),
    PRIMARY KEY (user_id, message_id)
);
