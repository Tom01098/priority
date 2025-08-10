-- Create the `created_at` column which defaults to the current timestamp when the row is inserted.
-- Existing rows are set to the current timestamp of the migration.
-- SQLite doesn't support updating the default value of a column,
-- so we have to create a new table and copy the data.

ALTER TABLE todo
    ADD COLUMN created_at TIMESTAMP;

UPDATE todo
SET created_at = CURRENT_TIMESTAMP;

CREATE TABLE todo_new
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    title      TEXT                                NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO todo_new (id, title, created_at)
SELECT id, title, created_at
FROM todo;

DROP TABLE todo;
ALTER TABLE todo_new
    RENAME TO todo;
