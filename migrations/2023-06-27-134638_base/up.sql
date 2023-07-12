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

CREATE TABLE courses (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  -- Year the course is created during
  year INTEGER UNSIGNED NOT NULL,
  name NVARCHAR(255) NOT NULL,

  -- Combination of year & name is unique
  CONSTRAINT uq_year_name UNIQUE (year, name),

  -- URL over which the course can be accessed
  -- VARCHAR because it can only be ASCII
  url VARCHAR(255) UNIQUE NOT NULL,

  professor INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_professor FOREIGN KEY (professor) REFERENCES users(id),

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

-- Copies all data for course being updated into courses_revisions table
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

CREATE TABLE point_assignments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  course INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_point_assignments_course FOREIGN KEY (course) REFERENCES courses(id),

  name NVARCHAR(255) UNIQUE NOT NULL,
  max_points INTEGER UNSIGNED NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE point_assignments_revisions (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  CONSTRAINT fk_point_assignments_revisions_id FOREIGN KEY (id) REFERENCES point_assignments(id),
  revision INTEGER UNSIGNED NOT NULL,
  CONSTRAINT PRIMARY KEY (id, revision),

  created DATETIME DEFAULT CURRENT_TIMESTAMP,

  course INTEGER UNSIGNED NOT NULL,
  name NVARCHAR(255) NOT NULL,
  max_points INTEGER UNSIGNED NOT NULL,
  deleted BOOL NOT NULL DEFAULT FALSE
);

-- Copies all data for point assignment being updated into point_assignments_revisions table
CREATE TRIGGER bu_point_assignments BEFORE UPDATE ON point_assignments FOR EACH ROW BEGIN
  INSERT INTO point_assignments_revisions (
    id,
    revision,
    created,
    course,
    name,
    max_points,
    deleted
  ) SELECT
    OLD.id,
    -- AUTO_INCREMENT the revision
    IFNULL(MAX(point_assignments_revisions.revision), 0) + 1,
    NOW(),
    OLD.course,
    OLD.name,
    OLD.max_points,
    OLD.deleted
    FROM point_assignments_revisions WHERE point_assignments_revisions.id = OLD.id;
END;

CREATE TABLE point_assignments_progress (
  assignment INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_point_assignments_progress_assignment FOREIGN KEY (assignment) REFERENCES point_assignments(id),
  student INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_point_assignments_progress_student FOREIGN KEY (student) REFERENCES users(id),
  CONSTRAINT PRIMARY KEY (assignment, student),

  points INTEGER UNSIGNED NOT NULL,
  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE point_assignments_progress_revisions (
  assignment INTEGER UNSIGNED NOT NULL,
  student INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_point_assignments_progress_revisions_id FOREIGN KEY (assignment, student) REFERENCES point_assignments_progress(assignment, student),
  revision INTEGER UNSIGNED NOT NULL,
  CONSTRAINT PRIMARY KEY (assignment, student, revision),

  created DATETIME DEFAULT CURRENT_TIMESTAMP,

  points INTEGER UNSIGNED NOT NULL,
  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TRIGGER bu_point_assignments_progress BEFORE UPDATE ON point_assignments_progress FOR EACH ROW BEGIN
  INSERT INTO point_assignments_progress_revisions (
    assignment,
    student,
    revision,
    created,
    points,
    deleted
  ) SELECT
    OLD.assignment,
    OLD.student,
    -- AUTO_INCREMENT the revision
    IFNULL(MAX(point_assignments_progress_revisions.revision), 0) + 1,
    NOW(),
    OLD.points,
    OLD.deleted
    FROM point_assignments_progress_revisions WHERE point_assignments_progress_revisions.assignment = OLD.assignment
                                              AND   point_assignments_progress_revisions.student = OLD.student;
END;


-- GRADE ASSIGNMENTS
CREATE TABLE grade_assignments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  course INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_grade_assignments_course FOREIGN KEY (course) REFERENCES courses(id),

  name NVARCHAR(255) UNIQUE NOT NULL,

  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE grade_assignments_revisions (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
  CONSTRAINT fk_grade_assignments_revisions_id FOREIGN KEY (id) REFERENCES grade_assignments(id),
  revision INTEGER UNSIGNED NOT NULL,
  CONSTRAINT PRIMARY KEY (id, revision),

  created DATETIME DEFAULT CURRENT_TIMESTAMP,

  course INTEGER UNSIGNED NOT NULL,
  name NVARCHAR(255) NOT NULL,
  deleted BOOL NOT NULL DEFAULT FALSE
);

-- Copies all data for point assignment being updated into grade_assignments_revisions table
CREATE TRIGGER bu_grade_assignments BEFORE UPDATE ON grade_assignments FOR EACH ROW BEGIN
  INSERT INTO grade_assignments_revisions (
    id,
    revision,
    created,
    course,
    name,
    deleted
  ) SELECT
    OLD.id,
    -- AUTO_INCREMENT the revision
    IFNULL(MAX(grade_assignments_revisions.revision), 0) + 1,
    NOW(),
    OLD.course,
    OLD.name,
    OLD.deleted
    FROM grade_assignments_revisions WHERE grade_assignments_revisions.id = OLD.id;
END;

CREATE TABLE grade_assignments_progress (
  assignment INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_grade_assignments_progress_assignment FOREIGN KEY (assignment) REFERENCES grade_assignments(id),
  student INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_grade_assignments_progress_student FOREIGN KEY (student) REFERENCES users(id),
  CONSTRAINT PRIMARY KEY (assignment, student),

  grade FLOAT NOT NULL,
  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE grade_assignments_progress_revisions (
  assignment INTEGER UNSIGNED NOT NULL,
  student INTEGER UNSIGNED NOT NULL,
  CONSTRAINT fk_grade_assignments_progress_revisions_id FOREIGN KEY (assignment, student) REFERENCES grade_assignments_progress(assignment, student),
  revision INTEGER UNSIGNED NOT NULL,
  CONSTRAINT PRIMARY KEY (assignment, student, revision),

  created DATETIME DEFAULT CURRENT_TIMESTAMP,

  grade FLOAT NOT NULL,
  deleted BOOL NOT NULL DEFAULT FALSE
);

CREATE TRIGGER bu_grade_assignments_progress BEFORE UPDATE ON grade_assignments_progress FOR EACH ROW BEGIN
  INSERT INTO grade_assignments_progress_revisions (
    assignment,
    student,
    revision,
    created,
    grade,
    deleted
  ) SELECT
    OLD.assignment,
    OLD.student,
    -- AUTO_INCREMENT the revision
    IFNULL(MAX(grade_assignments_progress_revisions.revision), 0) + 1,
    NOW(),
    OLD.grade,
    OLD.deleted
    FROM grade_assignments_progress_revisions WHERE grade_assignments_progress_revisions.assignment = OLD.assignment
                                              AND   grade_assignments_progress_revisions.student = OLD.student;
END;

