-- Your SQL goes here
CREATE TABLE dives (
  id SERIAL PRIMARY KEY,
  dive_id uuid UNIQUE NOT NULL,
  discipline_type TEXT,
  depth FLOAT,
  distance FLOAT,
  dive_time BIGINT,
  dive_name TEXT,
  dive_session uuid NOT NULL REFERENCES dive_sessions (session_id),
  user_id uuid NOT NULL REFERENCES users (user_id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  deleted_by uuid
);