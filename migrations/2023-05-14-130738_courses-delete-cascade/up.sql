ALTER TABLE grade_assignments
DROP CONSTRAINT `fk_grade_assignment_course`;
ALTER TABLE grade_assignments
ADD CONSTRAINT `fk_grade_assignment_course`
FOREIGN KEY (course) REFERENCES courses(id)
ON DELETE CASCADE;

ALTER TABLE point_assignments
DROP CONSTRAINT `fk_point_assignment_course`;
ALTER TABLE point_assignments
ADD CONSTRAINT `fk_point_assignment_course`
FOREIGN KEY (course) REFERENCES courses(id)
ON DELETE CASCADE;
