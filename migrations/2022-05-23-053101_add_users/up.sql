CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Your SQL goes here
CREATE TABLE users (
  username TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  password_salt bytea NOT NULL,
  email TEXT NOT NULL,
  last_login TIMESTAMPTZ NOT NULL,
  is_email_verified BOOLEAN NOT NULL,
  verified_date TIMESTAMPTZ,
  verification_code TEXT,
  verification_code_expiry TIMESTAMPTZ,
  -- default data
  -- id SERIAL PRIMARY KEY,
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMPTZ,
  archived_by uuid,
  PRIMARY KEY (id)
);