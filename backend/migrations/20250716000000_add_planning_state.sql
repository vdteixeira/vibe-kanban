-- Add 'planning' state to task status constraint
-- This migration adds the Planning state between Todo and InProgress in the task workflow

-- SQLite doesn't support DROP CONSTRAINT for inline constraints
-- We need to recreate the table with the new constraint

-- Create a new table with the updated constraint
CREATE TABLE tasks_new (
    id          BLOB PRIMARY KEY,
    project_id  BLOB NOT NULL,
    title       TEXT NOT NULL,
    description TEXT,
    status      TEXT NOT NULL DEFAULT 'todo'
                   CHECK (status IN ('todo','planning','inprogress','inreview','done','cancelled')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Copy data from old table to new table
INSERT INTO tasks_new SELECT * FROM tasks;

-- Drop the old table
DROP TABLE tasks;

-- Rename the new table
ALTER TABLE tasks_new RENAME TO tasks;