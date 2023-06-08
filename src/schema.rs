// @generated automatically by Diesel CLI.

diesel::table! {
    dive_sessions (id) {
        id -> Int4,
        session_id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        session_name -> Nullable<Text>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    dives (id) {
        id -> Int4,
        dive_id -> Uuid,
        discipline_type -> Nullable<Text>,
        depth -> Nullable<Float8>,
        distance -> Nullable<Float8>,
        dive_time -> Nullable<Int8>,
        dive_name -> Nullable<Text>,
        session_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Uuid,
        username -> Text,
        hashed_password -> Text,
        password_salt -> Bytea,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dive_sessions,
    dives,
    users,
);
