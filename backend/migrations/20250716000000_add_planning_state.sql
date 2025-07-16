-- Add 'planning' state to task status constraint
-- This migration adds the Planning state between Todo and InProgress in the task workflow

-- Drop the existing constraint
ALTER TABLE tasks DROP CONSTRAINT tasks_status_check;

-- Add the new constraint with 'planning' state included
ALTER TABLE tasks ADD CONSTRAINT tasks_status_check 
    CHECK (status IN ('todo','planning','inprogress','inreview','done','cancelled'));