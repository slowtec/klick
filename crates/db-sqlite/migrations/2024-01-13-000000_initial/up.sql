CREATE TABLE IF NOT EXISTS accounts (
    id              INTEGER NOT NULL,
    email           TEXT    NOT NULL COLLATE NOCASE,
    email_confirmed BOOLEAN NOT NULL,
    password        TEXT    NOT NULL, -- Password as bcrypt hash
    --
    UNIQUE          (email),
    --
    PRIMARY KEY     (id)
);

CREATE TABLE IF NOT EXISTS account_tokens (
    id            INTEGER NOT NULL,
    account_id    INTEGER NOT NULL,
    expires_at    INTEGER NOT NULL,
    nonce         TEXT    NOT NULL,
    --
    UNIQUE        (account_id),
    UNIQUE        (nonce),
    --
    PRIMARY KEY   (id),
    FOREIGN KEY   (account_id) REFERENCES accounts(id)
);
