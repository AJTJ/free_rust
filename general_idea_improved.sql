-- NOTE: Loggers are the loggers assembled by the user from a seletion of pre-defined categories and pre-defined fields. The user creates a logger(s) and then uses those created loggers to log dive data.
CREATE TABLE loggers (
  logger_name TEXT NOT NULL,
  -- relationships
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

-- I'm keeping this one type validation in the database. I could probably drop it.
CREATE TYPE PREDEFINED_INPUT_TYPES AS ENUM (
  'INTEGER',
  'ENUM',
  'INTERVAL',
  'TIMESTAMP',
  'TEXT'
);

-- Rather than having separate tables for the logger categories and the logger input types (that were very confusing)... I figured I can hold all the logger entry data (the fields that make up the logger form) in one spot.
CREATE TABLE logger_entries (
  item_order INTEGER,
  -- to be decided by the server?
  field_name TEXT NOT NULL,
  category_name TEXT NOT NULL,
  input_type PREDEFINED_INPUT_TYPES NOT NULL,
  -- relationships
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
);

-- The logs are the logs created BY the loggers.
CREATE TABLE all_logs (
  log_name TEXT,
  -- relationships
  session_id uuid REFERENCES dive_sessions (id),
  logger_used uuid NOT NULL REFERENCES loggers (id),
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

-- again like the logger_entries, it seems like all the log entries only really need to be in one table. I can enforce all the types etc... server-side.
CREATE TABLE log_entries (
  item_order INTEGER,
  -- Should this eventually be decided by the database?
  category_type TEXT NOT NULL,
  -- input data
  input_type PREDEFINED_INPUT_TYPES NOT NULL,
  -- TEXT to be parsed by the server?
  input_value TEXT,
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