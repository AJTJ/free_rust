-- Your SQL goes here
CREATE TABLE dives (
  -- dive specific information
  discipline_type TEXT,
  depth FLOAT,
  distance FLOAT,
  dive_time BIGINT,
  dive_name TEXT,
  -- relationship data
  session_id uuid NOT NULL REFERENCES apnea_sessions (id),
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