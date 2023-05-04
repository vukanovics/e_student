CREATE TABLE grade_assignments_progress (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  assignment INTEGER UNSIGNED NOT NULL,

  student INTEGER UNSIGNED NOT NULL,

  grade FLOAT NOT NULL,

  CONSTRAINT `fk_grade_assignments_progress_assignment` FOREIGN KEY (assignment) REFERENCES grade_assignments(id),
  CONSTRAINT `fk_grade_assignments_progress_student` FOREIGN KEY (student) REFERENCES users(id)
)
