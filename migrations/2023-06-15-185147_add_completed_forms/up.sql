-- Your SQL goes here
CREATE TABLE completed_forms (
  completed_form_name TEXT,
  template_version int [] NOT NULL,
  -- relationships
  original_form_id uuid NOT NULL REFERENCES forms (id),
  previous_completed_form_id uuid REFERENCES completed_forms (id),
  session_id uuid NOT NULL REFERENCES dive_sessions (id),
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