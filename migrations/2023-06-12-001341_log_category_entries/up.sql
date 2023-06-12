-- Your SQL goes here
CREATE TABLE log_category_entries (
  -- entry_specific_data
  -- NOTE: Is this table even necessary?
  item_order INTEGER,
  -- relationship data
  -- For getting the category info
  logger_category_type_id uuid NOT NULL REFERENCES logger_category_types (unique_id),
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