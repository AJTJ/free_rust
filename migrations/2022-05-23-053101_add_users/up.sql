-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_id uuid UNIQUE NOT NULL,
  username TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  password_salt bytea,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);