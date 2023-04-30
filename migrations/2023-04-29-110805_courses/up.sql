CREATE TABLE courses (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,

  year INTEGER UNSIGNED NOT NULL,
  name NVARCHAR(255) NOT NULL,

  -- Course name written in a way that only uses standard ASCII characters
  -- Used for the URL
  url VARCHAR(255) NOT NULL,

  -- Professor that owns the course
  professor INTEGER UNSIGNED NOT NULL,

  CONSTRAINT `fk_professor` FOREIGN KEY (professor) REFERENCES users(id)
)
