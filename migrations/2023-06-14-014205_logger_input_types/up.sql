-- Your SQL goes here
CREATE TYPE PREDEFINED_FIELD_NAMES AS ENUM (
  -- general
  'GENERAL_FEELING',
  -- sleep
  'SLEEP_START',
  'SLEEP_END',
  'AMOUNT_OF_SLEEP',
  -- food
  'LAST_MEAL',
  'LAST_MEAL_TIME',
  'COFFEE_QUANTITY',
  'QUALITY_LAST_MEAL'
);

CREATE TYPE PREDEFINED_INPUT_TYPES AS ENUM (
  'INTEGER',
  'ENUM',
  'INTERVAL',
  'TIMESTAMP',
  'TEXT'
);

CREATE TABLE logger_input_entries (
  item_order INTEGER,
  -- limited by database, enforced by server
  field_name PREDEFINED_FIELD_NAMES NOT NULL,
  -- limited by database, enforced by server
  category_type PREDEFINED_SESSION_CATEGORIES NOT NULL,
  -- limited by database, enforced by server
  input_type PREDEFINED_INPUT_TYPES NOT NULL,
  -- relationships
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
);