-- Your SQL goes here
CREATE TABLE loggers (
  logger_name TEXT NOT NULL,
  -- NOTE: We can make a join with the category_entries and the field_entries to see what the logger uses.
  -- relationships
  user_id uuid NOT NULL REFERENCES users (unique_id),
  -- default data
  id SERIAL PRIMARY KEY,
  unique_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
)