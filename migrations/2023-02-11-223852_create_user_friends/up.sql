-- Your SQL goes here
CREATE TABLE user_friends (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  friend_id INTEGER NOT NULL,
  request_accepted BOOLEAN NOT NULL DEFAULT 0
)
