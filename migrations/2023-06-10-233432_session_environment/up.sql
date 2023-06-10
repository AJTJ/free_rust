-- Your SQL goes here
CREATE TABLE session_environment (
  air_temperature INTEGER,
  water_current_strength INTEGER,
  water_temperature INTEGER,
  water_wave_strength INTEGER,
  -- people and animals
  buddy_qualification INTEGER,
  -- relationship data
  air_condition uuid REFERENCES air_weather_types (unique_id),
  session_id uuid NOT NULL REFERENCES dive_sessions (session_id),
  user_id uuid NOT NULL REFERENCES users (user_id),
  -- default data
  id SERIAL PRIMARY KEY,
  session_environment_id uuid UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL,
  deleted_at TIMESTAMP,
  deleted_by uuid
);