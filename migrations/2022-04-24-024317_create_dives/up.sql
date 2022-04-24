-- Your SQL goes here
CREATE TYPE discipline AS ENUM (
  'fim',
  'cwb',
  'cwt',
  'cnf',
  'dyn',
  'dnf',
  'se',
  'sta',
  'vwt',
  'nlt'
);

CREATE TABLE dives (
  id SERIAL PRIMARY KEY,
  discipline_type discipline,
  depth FLOAT,
  distance FLOAT,
  dive_time TIME,
  dive_name TEXT,
  dive_session INTEGER NOT NULL REFERENCES dive_sessions (id),
  user_id INTEGER NOT NULL REFERENCES users (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);