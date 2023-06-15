-- Your SQL goes here
CREATE TABLE logger_category_entries (
  -- other specific data
  item_order INTEGER,
  -- relationship data
  -- NOTE: An entry refers to its type
  logger_category_type_id uuid NOT NULL REFERENCES logger_category_types (id),
  -- NOTE: An entry ALWAYS exists in a list
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
)