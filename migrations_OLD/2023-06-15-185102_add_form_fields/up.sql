-- Your SQL goes here
CREATE TABLE form_fields (
  field_order INTEGER,
  -- field data
  field_name TEXT NOT NULL,
  field_value TEXT [],
  category_name TEXT NOT NULL,
  field_value_type TEXT [] NOT NULL,
  -- relationships
  form_id uuid NOT NULL REFERENCES forms (id),
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