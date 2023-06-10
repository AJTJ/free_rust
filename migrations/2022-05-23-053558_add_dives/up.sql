-- Your SQL goes here
CREATE TABLE dives (
  discipline_type TEXT,
  depth FLOAT,
  distance FLOAT,
  dive_time BIGINT,
  dive_name TEXT,
  -- relationship data
  session_id uuid NOT NULL REFERENCES dive_sessions (session_id),
  user_id uuid NOT NULL REFERENCES users (user_id),
  -- default data
  id SERIAL PRIMARY KEY,
  dive_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
);