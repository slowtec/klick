ALTER TABLE accounts       RENAME COLUMN id         TO rowid;
ALTER TABLE account_tokens RENAME COLUMN id         TO rowid;
ALTER TABLE account_tokens RENAME COLUMN account_id TO account_rowid;

CREATE TABLE IF NOT EXISTS projects (
    rowid         INTEGER NOT NULL,
    project_id    TEXT    NOT NULL,
    account_rowid INTEGER NOT NULL,
    data          JSON    NOT NULL, -- JSON serialized project data
    --
    UNIQUE        (project_id),
    --
    PRIMARY KEY   (rowid),
    FOREIGN KEY   (account_rowid) REFERENCES accounts(rowid)
);
