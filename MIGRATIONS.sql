-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Your SQL goes here
CREATE TABLE dive_sessions (
  id SERIAL PRIMARY KEY,
  start_time TIMESTAMP,
  end_time TIMESTAMP,
  session_name TEXT,
  user_id INTEGER NOT NULL REFERENCES users (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- this doesn't seem useful
-- Your SQL goes here
-- CREATE TYPE discipline AS ENUM (
--   'fim',
--   'cwb',
--   'cwt',
--   'cnf',
--   'dyn',
--   'dnf',
--   'se',
--   'sta',
--   'vwt',
--   'nlt'
-- );
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