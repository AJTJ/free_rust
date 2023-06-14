-- Your SQL goes here
CREATE TABLE all_logs (
  -- relationship data
  session_id uuid REFERENCES dive_sessions (unique_id),
  user_id uuid NOT NULL REFERENCES users (unique_id),
  logger_used uuid NOT NULL REFERENCES loggers (unique_id),
  -- default data
  id SERIAL PRIMARY KEY,
  unique_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
)