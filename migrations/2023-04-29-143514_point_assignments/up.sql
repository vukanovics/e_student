CREATE TABLE point_assignments (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  course INTEGER UNSIGNED NOT NULL,

  name NVARCHAR(255) NOT NULL,

  max_points INTEGER UNSIGNED NOT NULL,

  CONSTRAINT `fk_point_assignment_course` FOREIGN KEY (course) REFERENCES courses(id)
)
