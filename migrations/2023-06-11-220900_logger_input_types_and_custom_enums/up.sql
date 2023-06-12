-- Your SQL goes here
-- NOTE: This is specifically for the logger
CREATE TYPE CUSTOM_TYPE AS ENUM ('FLOAT', 'INTEGER', 'CUSTOM_ENUM');

-- Your SQL goes here
CREATE TABLE custom_enum_categories (
  category_name TEXT,
  -- default data
  id SERIAL PRIMARY KEY,
  unique_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
);

CREATE TABLE custom_enum_variants (
  variant_name TEXT,
  -- default data
  -- NOTE: An enum variant exists only in one custom_enum_category
  custom_enum_category_id uuid NOT NULL REFERENCES custom_enum_categories (unique_id),
  id SERIAL PRIMARY KEY,
  unique_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
);

CREATE TABLE logger_input_types (
  logger_input_name TEXT NOT NULL,
  input_value_type CUSTOM_TYPE NOT NULL,
  -- relationship data
  custom_enum_category_id uuid REFERENCES custom_enum_categories (unique_id),
  -- NOTE: input types exist only in one category
  logger_category_id uuid NOT NULL REFERENCES logger_category_types (unique_id),
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