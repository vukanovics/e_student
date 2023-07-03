CREATE TABLE users (
  id INTEGER UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,

  -- Password is stored as a bcrypt hash, 
  -- which has 60 characters
  password VARCHAR(60) NOT NULL,
  -- Type selected according to 
  -- https://stackoverflow.com/questions/9295513/nvarchar-for-email-addresses-in-sql-server
  email NVARCHAR(320) UNIQUE NOT NULL,
  -- Envisioned to have only three values (student, professor, admin)
  account_type TINYINT UNSIGNED NOT NULL,

  password_reset_required BOOL NOT NULL DEFAULT TRUE,

  first_name NVARCHAR(32),
  last_name NVARCHAR(32),

  -- Can be NULL, in which case the user hasn't logged in yet
  last_login_time DATETIME DEFAULT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  INDEX in_email (email)
);

-- Stores historic values of users
CREATE TABLE users_revisions (
  id INTEGER UNSIGNED NOT NULL,
  revision INTEGER UNSIGNED NOT NULL,
  CONSTRAINT PRIMARY KEY (id, revision),

  created DATETIME DEFAULT CURRENT_TIMESTAMP,

  password VARCHAR(60) NOT NULL,
  email NVARCHAR(320) NOT NULL,
  account_type TINYINT UNSIGNED NOT NULL,
  password_reset_required BOOL NOT NULL,
  first_name NVARCHAR(32),
  last_name NVARCHAR(32),
  last_login_time DATETIME,
  deleted BOOL NOT NULL,

  CONSTRAINT fk_id FOREIGN KEY (id) REFERENCES users(id)
);

-- Copies all data for user being updated into users_revisions table
CREATE TRIGGER bu_users BEFORE UPDATE ON users FOR EACH ROW BEGIN
  INSERT INTO users_revisions (
    id,
    revision,
    created,
    password,
    email,
    account_type,
    password_reset_required,
    first_name,
    last_name,
    last_login_time,
    deleted
  ) SELECT
    OLD.id,
    -- AUTO_INCREMENT the revision
    IFNULL(MAX(users_revisions.revision), 0) + 1,
    NOW(),
    OLD.password,
    OLD.email,
    OLD.account_type,
    OLD.password_reset_required,
    OLD.first_name,
    OLD.last_name,
    OLD.last_login_time,
    OLD.deleted
    FROM users_revisions WHERE users_revisions.id = OLD.id;
END;

CREATE TABLE programs (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  short_name NCHAR(2) NOT NULL,
  full_name NVARCHAR(64) NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  CONSTRAINT PRIMARY KEY (id, created),
  INDEX in_created (created DESC)
);

CREATE TABLE generations (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  year INTEGER UNSIGNED NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  CONSTRAINT PRIMARY KEY (id, created),
  INDEX in_created (created DESC)
);

CREATE TABLE indicies (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  program INTEGER UNSIGNED NOT NULL,
  generation INTEGER UNSIGNED NOT NULL,

  number INTEGER UNSIGNED NOT NULL,

  student INTEGER UNSIGNED NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  CONSTRAINT PRIMARY KEY (id, created),

  CONSTRAINT fk_program FOREIGN KEY (program) REFERENCES programs(id) ON DELETE CASCADE,
  CONSTRAINT fk_generation FOREIGN KEY (generation) REFERENCES generations(id) ON DELETE CASCADE,
  CONSTRAINT fk_student FOREIGN KEY (student) REFERENCES users(id) ON DELETE CASCADE,

  INDEX in_created (created DESC)
);

CREATE TABLE sessions (
  session_key BINARY(32) PRIMARY KEY,

  user INTEGER UNSIGNED NOT NULL,
  created_on DATETIME NOT NULL,
  last_refreshed DATETIME NOT NULL,
  timeout_duration_seconds INT UNSIGNED NOT NULL,

  CONSTRAINT fk_user FOREIGN KEY (user) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE courses (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  year INTEGER UNSIGNED NOT NULL,
  name NVARCHAR(255) NOT NULL,

  url VARCHAR(255) NOT NULL,

  professor INTEGER UNSIGNED NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  CONSTRAINT PRIMARY KEY (id, created),

  CONSTRAINT fk_professor FOREIGN KEY (professor) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE enrolments (
  course INTEGER UNSIGNED NOT NULL,
  student INTEGER UNSIGNED NOT NULL,

  CONSTRAINT PRIMARY KEY (course, student),

  CONSTRAINT fk_enrolments_course FOREIGN KEY (course) REFERENCES courses(id) ON DELETE CASCADE,
  CONSTRAINT fk_enrolments_student FOREIGN KEY (student) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE point_assignments (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  course INTEGER UNSIGNED NOT NULL,

  name NVARCHAR(255) NOT NULL,
  max_points INTEGER UNSIGNED NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  CONSTRAINT PRIMARY KEY (id, created),

  CONSTRAINT fk_point_assignments_course FOREIGN KEY (course) REFERENCES courses(id) ON DELETE CASCADE
);

CREATE TABLE point_assignments_progress (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  assignment INTEGER UNSIGNED NOT NULL,

  student INTEGER UNSIGNED NOT NULL,

  points INTEGER UNSIGNED NOT NULL,

  CONSTRAINT PRIMARY KEY (id, created),

  CONSTRAINT fk_point_assignments_progress_assignment FOREIGN KEY (assignment) REFERENCES point_assignments(id) ON DELETE CASCADE,
  CONSTRAINT fk_point_assignments_progress_student FOREIGN KEY (student) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE grade_assignments (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  course INTEGER UNSIGNED NOT NULL,

  name NVARCHAR(255) NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  CONSTRAINT PRIMARY KEY (id, created),

  CONSTRAINT fk_grade_assignments_course FOREIGN KEY (course) REFERENCES courses(id) ON DELETE CASCADE
);

CREATE TABLE grade_assignments_progress (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  assignment INTEGER UNSIGNED NOT NULL,

  student INTEGER UNSIGNED NOT NULL,

  grade FLOAT NOT NULL,

  CONSTRAINT PRIMARY KEY (id, created),

  CONSTRAINT fk_grade_assignments_progress_assignment FOREIGN KEY (assignment) REFERENCES grade_assignments(id) ON DELETE CASCADE,
  CONSTRAINT fk_grade_assignments_progress_student FOREIGN KEY (student) REFERENCES users(id) ON DELETE CASCADE
);
