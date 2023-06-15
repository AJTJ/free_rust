-- Your SQL goes here
CREATE TABLE loggers (
  logger_name TEXT NOT NULL,
  -- relationships
  user_id uuid NOT NULL REFERENCES users (id),
  -- user_defs will contain things like order of the logger fields
  user_defs jsonb,
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid,
  PRIMARY KEY (id)
)