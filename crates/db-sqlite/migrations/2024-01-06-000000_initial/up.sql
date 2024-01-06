CREATE TABLE IF NOT EXISTS users (
  email           TEXT NOT NULL, -- email address = User ID = user name for login
  email_confirmed BOOL NOT NULL, -- if the email address was confirmed
  password        TEXT,          -- Password as bcrypt hash
  --
  PRIMARY KEY (email)
);
