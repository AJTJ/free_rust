-- Your SQL goes here
CREATE TABLE logger_category_types (
  logger_category_name TEXT NOT NULL,
  -- relationship data
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid,
  PRIMARY KEY (id)
)