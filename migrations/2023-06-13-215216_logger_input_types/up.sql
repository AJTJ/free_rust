-- Your SQL goes here
-- CREATE TABLE sleep_input_type (
--   amount_of_sleep INTERVAL,
--   sleep_start TIMESTAMP,
--   sleep_end TIMESTAMP,
--   general_quality_of_sleep INTEGER,
--   -- TODO: sleep_events SLEEP_EVENTS_ENUM,
--   -- default data
--   id SERIAL PRIMARY KEY,
--   unique_id uuid UNIQUE NOT NULL,
--   created_at TIMESTAMP NOT NULL,
--   updated_at TIMESTAMP NOT NULL,
--   is_active BOOLEAN NOT NULL,
--   deleted_at TIMESTAMP,
--   deleted_by uuid
-- )
CREATE TYPE PREDEFINED_SLEEP_FIELDS AS ENUM ('AMOUNT_OF_SLEEP', 'SLEEP_START');

CREATE TABLE sleep_input_type (
  input_type numeric GENERATED ALWAYS AS (height_cm / 2.54) STORED,
  -- default data
  id SERIAL PRIMARY KEY,
  unique_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
)