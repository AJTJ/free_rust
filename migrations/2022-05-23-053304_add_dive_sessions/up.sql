-- Your SQL goes here
CREATE TABLE dive_sessions (
  -- session-specific info
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL,
  session_name TEXT,
  -- relationship data
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);