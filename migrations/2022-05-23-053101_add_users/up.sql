-- Your SQL goes here
CREATE TABLE users (
  username TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  password_salt bytea NOT NULL,
  email TEXT NOT NULL,
  last_login TIMESTAMP NOT NULL,
  -- default data
  id SERIAL PRIMARY KEY,
  user_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
);