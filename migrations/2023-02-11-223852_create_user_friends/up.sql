-- Your SQL goes here
CREATE TABLE user_friends (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users (id),
    friend_id INTEGER NOT NULL REFERENCES users (id),
    request_accepted BOOLEAN NOT NULL DEFAULT 0
)