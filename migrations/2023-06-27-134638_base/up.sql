CREATE TABLE users (
  id INTEGER UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,

  -- Password is stored as a bcrypt hash, 
  -- which has 60 characters
  password VARCHAR(60) NOT NULL,
  -- Type selected according to 
  -- https://stackoverflow.com/questions/9295513/nvarchar-for-email-addresses-in-sql-server
  email NVARCHAR(320) UNIQUE NOT NULL,
  account_type TINYINT UNSIGNED NOT NULL,

  password_reset_required BOOL NOT NULL DEFAULT TRUE,

  first_name NVARCHAR(32),
  last_name NVARCHAR(32),

  last_login_time DATETIME DEFAULT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE,

  INDEX in_email (email)
);

CREATE TABLE users_revisions (
  id INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_users_revisions_id FOREIGN KEY (id) REFERENCES users(id),
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
  deleted BOOL NOT NULL
);

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
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  short_name NCHAR(2) UNIQUE NOT NULL,
  full_name NVARCHAR(64) UNIQUE NOT NULL
);

CREATE TABLE generations (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  year INTEGER UNSIGNED UNIQUE NOT NULL
);

CREATE TABLE indicies (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  program INTEGER UNSIGNED NOT NULL,
  generation INTEGER UNSIGNED NOT NULL,
  number INTEGER UNSIGNED NOT NULL,

  CONSTRAINT uq_program_generation_number UNIQUE (program, generation, number),

  student INTEGER UNSIGNED UNIQUE NOT NULL,

  CONSTRAINT fk_program FOREIGN KEY (program) REFERENCES programs(id) ON DELETE CASCADE,
  CONSTRAINT fk_generation FOREIGN KEY (generation) REFERENCES generations(id) ON DELETE CASCADE,
  CONSTRAINT fk_student FOREIGN KEY (student) REFERENCES users(id)
);

CREATE TABLE sessions (
  session_key BINARY(32) PRIMARY KEY,

  user INTEGER UNSIGNED NOT NULL,
  created_on DATETIME NOT NULL,
  last_refreshed DATETIME NOT NULL,
  timeout_duration_seconds INT UNSIGNED NOT NULL,

  CONSTRAINT fk_user FOREIGN KEY (user) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE discussions (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  markdown TEXT
);

CREATE TABLE comments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  discussion INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_comment_discussion FOREIGN KEY (discussion) REFERENCES discussions(id),

  author INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_comment_author FOREIGN KEY (author) REFERENCES users(id),

  markdown TEXT NOT NULL,

  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE courses (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  year INTEGER UNSIGNED NOT NULL,
  name NVARCHAR(255) NOT NULL,

  CONSTRAINT uq_year_name UNIQUE (year, name),

  url VARCHAR(255) UNIQUE NOT NULL,

  professor INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_professor FOREIGN KEY (professor) REFERENCES users(id),

  discussion INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_course_discussion FOREIGN KEY (discussion) REFERENCES discussions(id),

  deleted BOOL NOT NULL DEFAULT FALSE,

  INDEX in_url (url)
);

CREATE TABLE courses_revisions (
  id INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_courses_revisions_id FOREIGN KEY (id) REFERENCES courses(id),

  revision INTEGER UNSIGNED NOT NULL,
  CONSTRAINT PRIMARY KEY (id, revision),

  created DATETIME DEFAULT CURRENT_TIMESTAMP,

  year INTEGER UNSIGNED NOT NULL,
  name NVARCHAR(255) NOT NULL,
  url VARCHAR(255) NOT NULL,
  professor INTEGER UNSIGNED NOT NULL,
  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TRIGGER bu_courses BEFORE UPDATE ON courses FOR EACH ROW BEGIN
  INSERT INTO courses_revisions (
    id,
    revision,
    created,
    year,
    name,
    url,
    professor,
    deleted
  ) SELECT
    OLD.id,
    -- AUTO_INCREMENT the revision
    IFNULL(MAX(courses_revisions.revision), 0) + 1,
    NOW(),
    OLD.year,
    OLD.name,
    OLD.url,
    OLD.professor,
    OLD.deleted
    FROM courses_revisions WHERE courses_revisions.id = OLD.id;
END;

CREATE TABLE enrolments (
  course INTEGER UNSIGNED NOT NULL,
  student INTEGER UNSIGNED NOT NULL,

  CONSTRAINT PRIMARY KEY (course, student),

  CONSTRAINT fk_enrolments_course FOREIGN KEY (course) REFERENCES courses(id) ON DELETE CASCADE,
  CONSTRAINT fk_enrolments_student FOREIGN KEY (student) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE assignments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  course INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_assignments_course FOREIGN KEY (course) REFERENCES courses(id),

  name NVARCHAR(255) NOT NULL,
  CONSTRAINT uq_assignments_name UNIQUE (course, name),
  url VARCHAR(255) NOT NULL,
  CONSTRAINT uq_assignments_url UNIQUE (course, url),

  discussion INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_assignment_discussion FOREIGN KEY (discussion) REFERENCES discussions(id),

  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE point_assignments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,
  assignment INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_point_assignments_assignment FOREIGN KEY (assignment) REFERENCES assignments(id),

  max_points INTEGER UNSIGNED NOT NULL
);

CREATE TABLE point_assignments_progress (
  assignment INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_point_assignments_progress_assignment FOREIGN KEY (assignment) REFERENCES point_assignments(id),
  student INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_point_assignments_progress_student FOREIGN KEY (student) REFERENCES users(id),
  CONSTRAINT PRIMARY KEY (assignment, student),

  points INTEGER UNSIGNED NOT NULL
);

CREATE TABLE grade_assignments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,
  assignment INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_grade_assignments_assignment FOREIGN KEY (assignment) REFERENCES assignments(id)
);

CREATE TABLE grade_assignments_progress (
  assignment INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_grade_assignments_progress_assignment FOREIGN KEY (assignment) REFERENCES grade_assignments(id),
  student INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_grade_assignments_progress_student FOREIGN KEY (student) REFERENCES users(id),
  CONSTRAINT PRIMARY KEY (assignment, student),

  grade_major TINYINT UNSIGNED NOT NULL,
  grade_minor TINYINT UNSIGNED NOT NULL
);

