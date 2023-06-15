-- Your SQL goes here
CREATE TYPE PREDEFINED_INPUT_TYPES AS ENUM (
  'INTEGER',
  'ENUM',
  'INTERVAL',
  'TIMESTAMP',
  'TEXT'
);

CREATE TABLE logger_entries (
  item_order INTEGER,
  -- to be decided by the server?
  field_name TEXT NOT NULL,
  category_name TEXT NOT NULL,
  input_type PREDEFINED_INPUT_TYPES NOT NULL,
  -- relationships
  logger_id uuid NOT NULL REFERENCES loggers (id),
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid,
  PRIMARY KEY (id)
);