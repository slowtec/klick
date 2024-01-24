PRAGMA foreign_keys=off;

-- We don't need to start a transaction here
-- because diesel executes the migrations
-- in a transaction anyway :)

ALTER TABLE accounts
ADD COLUMN created_at INTEGER NOT NULL DEFAULT 0;

-- Added 'ON DELETE CASCADE'
-- to the foreign key 'account_rowid'
CREATE TABLE account_tokens_new (
    rowid         INTEGER NOT NULL,
    account_rowid INTEGER NOT NULL,
    expires_at    INTEGER NOT NULL,
    nonce         TEXT    NOT NULL,
    --
    UNIQUE        (account_rowid),
    UNIQUE        (nonce),
    --
    PRIMARY KEY   (rowid),
    FOREIGN KEY   (account_rowid) REFERENCES accounts(rowid) ON DELETE CASCADE
);

-- Added 'ON DELETE CASCADE'
-- to the foreign key 'account_rowid'
CREATE TABLE projects_new (
    rowid         INTEGER NOT NULL,
    project_id    TEXT    NOT NULL,
    account_rowid INTEGER NOT NULL,
    data          JSON    NOT NULL, -- JSON serialized project data
    --
    UNIQUE        (project_id),
    --
    PRIMARY KEY   (rowid),
    FOREIGN KEY   (account_rowid) REFERENCES accounts(rowid) ON DELETE CASCADE
);

INSERT INTO account_tokens_new
SELECT * FROM account_tokens;

INSERT INTO projects_new
SELECT * FROM projects;

DROP TABLE account_tokens;
ALTER TABLE account_tokens_new RENAME TO account_tokens;

DROP TABLE projects;
ALTER TABLE projects_new RENAME TO projects;

PRAGMA foreign_keys=on;
