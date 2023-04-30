CREATE TABLE grade_assignments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  course INTEGER UNSIGNED NOT NULL,

  name NVARCHAR(255) NOT NULL,

  CONSTRAINT `fk_grade_assignment_course` FOREIGN KEY (course) REFERENCES courses(id)
)
