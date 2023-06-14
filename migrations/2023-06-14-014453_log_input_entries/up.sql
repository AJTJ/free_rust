-- -- Your SQL goes here
CREATE TABLE valid_enum_inputs (input_name TEXT PRIMARY KEY);

INSERT INTO
  valid_enum_inputs (input_name)
VALUES
  -- food
  ('OILY'),
  ('LIGHT'),
  ('FISH'),
  ('SORT OF GREASY'),
  ('ACIDIC - MILD'),
  ('ACIDIC - MODERATE'),
  ('ACIDIC - HEAVY');

CREATE TABLE log_input_entries (
  item_order INTEGER,
  -- limited by database, enforced by server
  input_value_type PREDEFINED_INPUT_TYPES NOT NULL,
  -- limited by database, enforced by server
  category_type PREDEFINED_SESSION_CATEGORIES NOT NULL,
  /*
   TODO: input_enum is not validated at all by the database
   I suppose I could ahve a big list of all possible enums
   But that feels clunky
   this could simply be validated through the server...
   */
  -- This could potentially also work, if I want to validate it on the backend somehow
  input_enum TEXT REFERENCES valid_enum_inputs (input_name) ON UPDATE CASCADE,
  -- input_enum TEXT,
  input_integer INTEGER,
  input_interval INTERVAL,
  input_timestamp TIMESTAMP,
  input_text TEXT,
  -- NOTE: This data is a string, but it will be defined elsewhere
  -- relationship data
  log_id uuid NOT NULL REFERENCES all_logs (unique_id),
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