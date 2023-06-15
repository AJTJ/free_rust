-- Your SQL goes here
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