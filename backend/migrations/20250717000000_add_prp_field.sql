-- Add PRP (Product Requirements Planning) field to tasks
-- This field will store planning requirements created during the Planning phase

ALTER TABLE tasks ADD COLUMN prp TEXT DEFAULT NULL;