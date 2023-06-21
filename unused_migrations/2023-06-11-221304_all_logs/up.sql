-- Your SQL goes here
CREATE TABLE all_logs (
  -- relationship data
  session_id uuid REFERENCES dive_sessions (id),
  dive_id uuid REFERENCES dives (id),
  user_id uuid NOT NULL REFERENCES users (id),
  logger_used uuid NOT NULL REFERENCES loggers (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
)