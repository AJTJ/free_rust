CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Your SQL goes here
CREATE TABLE users (
  username TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  password_salt bytea NOT NULL,
  email TEXT NOT NULL,
  last_login TIMESTAMP NOT NULL,
  -- default data
  -- id SERIAL PRIMARY KEY,
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);