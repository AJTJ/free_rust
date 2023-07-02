-- Your SQL goes here
CREATE TABLE reports (
  report_data jsonb NOT NULL,
  -- relationships
  form_id uuid NOT NULL REFERENCES forms (id),
  original_form_id uuid REFERENCES forms (id),
  previous_report_id uuid REFERENCES reports (id),
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