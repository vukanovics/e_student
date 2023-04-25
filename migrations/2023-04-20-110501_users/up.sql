CREATE TABLE users (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  -- Password is stored as a bcrypt hash, 
  -- which has 60 characters
  password VARCHAR(60) NOT NULL,
  -- Type selected according to 
  -- https://stackoverflow.com/questions/9295513/nvarchar-for-email-addresses-in-sql-server
  email NVARCHAR(320) UNIQUE NOT NULL,
  -- Envisioned to have only three values (student, professor, admin)
  account_type TINYINT UNSIGNED NOT NULL,
  -- Used as a boolean value for whether the user needs to set a password on the
  -- next login
  password_reset_required BOOL NOT NULL,
  -- User sets the username on the first log in - before that, it's NULL
  -- Max length is chosen arbitrarily
  username NVARCHAR(32) UNIQUE,
  -- Can be NULL, in which case the user hasn't logged in yet
  last_login_time DATETIME
)
