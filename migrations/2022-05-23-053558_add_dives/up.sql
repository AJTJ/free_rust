-- Your SQL goes here
CREATE TABLE dives (
  id SERIAL PRIMARY KEY,
  discipline_type TEXT,
  depth FLOAT,
  distance FLOAT,
  dive_time TIME,
  dive_name TEXT,
  dive_session INTEGER NOT NULL REFERENCES dive_sessions (id),
  user_id INTEGER NOT NULL REFERENCES users (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);