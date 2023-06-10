-- Your SQL goes here
CREATE TYPE air_weather_type AS ENUM ('rain', 'snow', 'dust', 'volcano');

CREATE TYPE social_event_type AS ENUM ('annoying_person', 'someone_injured');

CREATE TABLE session_environment (
  air_condition air_weather_type,
  air_temperature INTEGER,
  water_current_strength INTEGER,
  water_temperature INTEGER,
  water_wave_strength INTEGER,
  -- people and animals
  buddy_qualification INTEGER,
  social_event social_event_type,
  -- relationship data
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