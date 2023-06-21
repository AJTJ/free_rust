-- Your SQL goes here
CREATE TABLE logger_input_entries (
  -- other specific data
  item_order INTEGER,
  -- relationship data
  -- NOTE: An entry refers to its type
  logger_input_type_id uuid NOT NULL REFERENCES logger_input_types (id),
  -- NOTE: An entry exists in a list
  logger_category_entry_id uuid NOT NULL REFERENCES logger_category_entries (id),
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
)