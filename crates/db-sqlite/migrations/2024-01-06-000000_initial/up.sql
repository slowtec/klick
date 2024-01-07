CREATE TABLE IF NOT EXISTS accounts (
  email           TEXT NOT NULL, -- email address
  email_confirmed BOOL NOT NULL, -- whether the email address has been confirmed
  password        TEXT,          -- Password as bcrypt hash
  --
  PRIMARY KEY (email)
);
