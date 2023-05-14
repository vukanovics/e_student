ALTER TABLE point_assignments_progress
DROP CONSTRAINT `fk_point_assignments_progress_student`;
ALTER TABLE point_assignments_progress
ADD CONSTRAINT `fk_point_assignments_progress_student`
FOREIGN KEY (student) REFERENCES users(id);

ALTER TABLE grade_assignments_progress
DROP CONSTRAINT `fk_grade_assignments_progress_student`;
ALTER TABLE grade_assignments_progress
ADD CONSTRAINT `fk_grade_assignments_progress_student`
FOREIGN KEY (student) REFERENCES users(id);

ALTER TABLE enrolments
DROP CONSTRAINT `fk_student`;
ALTER TABLE enrolments
ADD CONSTRAINT `fk_student`
FOREIGN KEY (student) REFERENCES users(id);

ALTER TABLE sessions
DROP CONSTRAINT `fk_user_id`;
ALTER TABLE sessions
ADD CONSTRAINT `fk_user_id`
FOREIGN KEY (user_id) REFERENCES users(id);

ALTER TABLE courses
DROP CONSTRAINT `fk_professor`;
ALTER TABLE courses
ADD CONSTRAINT `fk_professor`
FOREIGN KEY (professor) REFERENCES users(id);
