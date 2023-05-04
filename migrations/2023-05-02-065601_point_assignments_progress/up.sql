CREATE TABLE point_assignments_progress (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  assignment INTEGER UNSIGNED NOT NULL,

  student INTEGER UNSIGNED NOT NULL,

  points INTEGER UNSIGNED NOT NULL,

  CONSTRAINT `fk_point_assignments_progress_assignment` FOREIGN KEY (assignment) REFERENCES point_assignments(id),
  CONSTRAINT `fk_point_assignments_progress_student` FOREIGN KEY (student) REFERENCES users(id)
)
