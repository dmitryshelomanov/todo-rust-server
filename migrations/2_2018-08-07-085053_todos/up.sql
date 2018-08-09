CREATE TABLE todos (
  id INT AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  checked BOOLEAN NOT NULL,
  user_id INT NOT NULL,
  PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id)
)