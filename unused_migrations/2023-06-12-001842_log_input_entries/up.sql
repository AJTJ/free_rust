CREATE TABLE log_input_entries (
  -- other specific data
  item_order INTEGER,
  -- NOTE: This data is a string, but it will be defined elsewhere
  input_type_used CUSTOM_TYPE NOT NULL,
  input_float FLOAT,
  input_integer INTEGER,
  -- type relationships
  custom_enum_category_id uuid REFERENCES custom_enum_categories (id),
  custom_enum_variant_id uuid REFERENCES custom_enum_variants (id),
  -- relationship data
  -- For getting the category
  log_category_entry_id uuid NOT NULL REFERENCES log_category_entries (id),
  -- For getting all the other input data
  logger_input_type_id uuid NOT NULL REFERENCES logger_input_types (id),
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