CREATE TABLE sessions (
  id INT AUTO_INCREMENT,
  token VARCHAR(255),
  user_id INT,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES users(id)
)
