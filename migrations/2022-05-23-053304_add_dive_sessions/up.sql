-- Your SQL goes here
CREATE TABLE dive_sessions (
  id SERIAL PRIMARY KEY,
  session_id uuid UNIQUE,
  start_time TIMESTAMP,
  end_time TIMESTAMP,
  session_name TEXT,
  user_id uuid NOT NULL REFERENCES users (user_id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);