-- NOTE: Loggers are the loggers assembled by the user from a seletion of pre-defined categories and pre-defined fields.
CREATE TABLE loggers (
  logger_name TEXT NOT NULL,
  -- relationships
  user_id uuid NOT NULL REFERENCES users (id),
  user_defs jsonb,
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);

-- This is related to the next table.
CREATE TYPE PREDEFINED_SESSION_CATEGORIES AS ENUM (
  'DIVE_SUMMARY',
  'GENERAL_FEELING',
  'HEALTH',
  'ENVIRONMENT',
  'SLEEP',
  'FOOD',
  'EXERTION',
  'PREVIOUS_DAY'
);

-- The logger_category_entries are the categories present in a given "logger".
-- the thinking behind having this was that I could store entry-specific details for each category, things like: order. But so far it only seems like order is the most relevant thing.
CREATE TABLE logger_category_entries (
  -- other specific data
  item_order INTEGER,
  -- relationship data
  -- NOTE: An entry refers to its type
  logger_category_type PREDEFINED_SESSION_CATEGORIES NOT NULL,
  -- TODO: for the future?
  -- custom_logger_category_type_id REFERENCES custom_logger_category_types (id),
  -- NOTE: An entry ALWAYS exists in a list
  logger_id uuid NOT NULL REFERENCES loggers (id),
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);

-- related to the logger_input_entries
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
  'QUALITY_LAST_MEAL',
);

CREATE TYPE PREDEFINED_INPUT_TYPES AS ENUM (
  'INTEGER',
  'ENUM',
  'INTERVAL',
  'TIMESTAMP',
  'TEXT'
);

-- Similar to the logger_category_entries, the logger_input_entries are the fields present in a logger assembled by the user.
-- Again like the logger_category_entries, the thinking was that I could then store specific infortion to the presence in the logger.
CREATE TABLE logger_input_entries (
  item_order INTEGER,
  -- limited by database enum, enforced by server
  field_name PREDEFINED_FIELD_NAMES NOT NULL,
  -- limited by database enum, enforced by server
  category_type PREDEFINED_SESSION_CATEGORIES NOT NULL,
  -- limited by database enum, enforced by server
  input_type PREDEFINED_INPUT_TYPES NOT NULL,
  -- relationships
  logger_id uuid NOT NULL REFERENCES loggers (id),
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);

-- Here we get to the actual logs themselves
-- The user has created loggers, and then they use them to log their dives.
-- I wasn't thinking of storing much data on here, since I would be performing jois to get all the log_input_entries related to a given log.
CREATE TABLE all_logs (
  -- relationship data
  session_id uuid REFERENCES dive_sessions (id),
  user_id uuid NOT NULL REFERENCES users (id),
  logger_used uuid NOT NULL REFERENCES loggers (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);

/*
 -- This is an example of validating enum types for the following table
 --
 --
 -- CREATE TABLE valid_enum_inputs (input_name TEXT PRIMARY KEY);
 -- INSERT INTO
 --   valid_enum_inputs (input_name)
 -- VALUES
 --   ('OILY'),
 --   ('LIGHT'),
 --   ('HEAVY');
 --
 --
 */
--
-- And finally we get to the log input entries themselves.
-- As can be imagined, these are the actual entries in any log
CREATE TABLE log_input_entries (
  item_order INTEGER,
  -- limited by database enum, enforced by server
  input_value_type PREDEFINED_INPUT_TYPES NOT NULL,
  -- limited by database enum, enforced by server
  category_type PREDEFINED_SESSION_CATEGORIES NOT NULL,
  /*
   NOTE: input_enum is not validated at all by the database
   but this could simply be validated through the server...
   */
  input_enum TEXT,
  /*
   I suppose I could ahve a big list of all possible enums. But that feels clunky
   It could look something like so:
   -- input_enum TEXT REFERENCES valid_enum_inputs (input_name) ON UPDATE CASCADE,
   */
  /* 
   This seems weird/silly to have columns for all the various data types that a log_input_entry COULD be...
   I'm wondering if it should just be TEXT and then have the server parse it correctly?
   */
  input_integer INTEGER,
  input_interval INTERVAL,
  input_timestamp TIMESTAMP,
  input_text TEXT,
  -- relationship data
  log_id uuid NOT NULL REFERENCES all_logs (id),
  user_id uuid NOT NULL REFERENCES users (id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);