-- -- Your SQL goes here
/*
 -
 *FUTURE THOUGHTS
 It seems unnecessary and hard to enforce the enum types in the database, with something like this:
 -- CREATE TABLE valid_enum_inputs (input_name TEXT PRIMARY KEY);
 -- INSERT INTO
 --   valid_enum_inputs (input_name)
 -- VALUES
 --   ('OILY'),
 --   ('LIGHT');
 That being said, it seems possible to enforce it here at a later date, if necessary.
 But realistically. I should only have one server interacting with the database at any given time, anyways.
 So it's fair to assume that this everything will ALWAYS go through a portal to the database.
 -
 */
CREATE TABLE valid_enum_inputs (input_name TEXT PRIMARY KEY);

INSERT INTO
  valid_enum_inputs (input_name)
VALUES
  ('OILY'),
  ('LIGHT'),
  ('HEAVY');

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
  log_id uuid NOT NULL REFERENCES all_logs (id),
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid,
  PRIMARY KEY (id)
);