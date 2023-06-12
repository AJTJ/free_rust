-- Your SQL goes here
CREATE TABLE dive_sessions (
  -- session-specific info
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL,
  session_name TEXT,
  -- relationship data
  user_id uuid NOT NULL REFERENCES users (unique_id),
  -- default data
  id SERIAL PRIMARY KEY,
  unique_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
);