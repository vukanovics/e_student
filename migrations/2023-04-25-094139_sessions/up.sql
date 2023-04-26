CREATE TABLE sessions (
  -- Session key is 32 random bytes
  session_key BINARY(32) PRIMARY KEY,
  -- Constrained to id column of users
  user_id INTEGER UNSIGNED NOT NULL,
  -- When was this session first created
  created_on DATETIME NOT NULL,
  -- When was this session last refreshed
  last_refreshed DATETIME NOT NULL,
  -- How long since last_refreshed is this session valid for, in seconds
  timeout_duration_seconds INT UNSIGNED NOT NULL,

  CONSTRAINT `fk_user_id` FOREIGN KEY (user_id) REFERENCES users(id)
)
