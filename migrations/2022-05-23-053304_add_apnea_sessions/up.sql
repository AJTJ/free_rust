-- Your SQL goes here
CREATE TABLE apnea_sessions (
  -- session-specific info
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ,
  session_name TEXT,
  -- relationship data
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMPTZ,
  archived_by uuid,
  PRIMARY KEY (id)
);