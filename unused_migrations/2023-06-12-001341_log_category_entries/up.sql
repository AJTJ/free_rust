-- Your SQL goes here
CREATE TABLE log_category_entries (
  -- entry_specific_data
  -- NOTE: Is this table even necessary?
  item_order INTEGER,
  -- relationship data
  -- For getting the category info
  logger_category_type_id uuid NOT NULL REFERENCES logger_category_types (id),
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