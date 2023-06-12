-- Your SQL goes here
CREATE TABLE logger_category_entries (
  -- other specific data
  item_order INTEGER,
  -- relationship data
  -- NOTE: An entry refers to its type
  logger_category_type_id uuid NOT NULL REFERENCES logger_category_types (unique_id),
  -- NOTE: An entry exists in a list
  logger_id uuid NOT NULL REFERENCES loggers (unique_id),
  user_id uuid NOT NULL REFERENCES users (unique_id),
  -- default data
  id SERIAL PRIMARY KEY,
  unique_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
)