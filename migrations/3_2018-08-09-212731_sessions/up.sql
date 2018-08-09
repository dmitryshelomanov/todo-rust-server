CREATE TABLE sessions (
  id INT AUTO_INCREMENT,
  token VARCHAR(255) NOT NULL,
  user_id INT NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES users(id)
)
