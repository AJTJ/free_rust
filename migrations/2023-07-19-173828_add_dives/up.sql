-- Your SQL goes here
CREATE TABLE unique_apneas (
  -- dive specific information
  activity_type TEXT NOT NULL,
  activity_data jsonb NOT NULL,
  -- relationship data
  session_id uuid NOT NULL REFERENCES apnea_sessions (id),
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