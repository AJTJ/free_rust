// @generated automatically by Diesel CLI.

diesel::table! {
    dive_sessions (id) {
        id -> Int4,
        session_id -> Nullable<Uuid>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        session_name -> Nullable<Text>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    dives (id) {
        id -> Int4,
        dive_id -> Nullable<Uuid>,
        discipline_type -> Nullable<Text>,
        depth -> Nullable<Float8>,
        distance -> Nullable<Float8>,
        dive_time -> Nullable<Time>,
        dive_name -> Nullable<Text>,
        dive_session -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Uuid,
        username -> Text,
        hashed_password -> Text,
        password_salt -> Nullable<Bytea>,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dive_sessions,
    dives,
    users,
);
