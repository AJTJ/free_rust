-- Your SQL goes here
CREATE TABLE forms (
  form_name TEXT NOT NULL,
  template_version int [] NOT NULL,
  -- relationships
  user_id uuid NOT NULL REFERENCES users (id),
  original_form_id uuid REFERENCES forms(id),
  previous_form_id uuid REFERENCES forms(id),
  -- default data
  id uuid DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  archived_at TIMESTAMP,
  archived_by uuid,
  PRIMARY KEY (id)
);