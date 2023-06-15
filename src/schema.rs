// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "predefined_input_types"))]
    pub struct PredefinedInputTypes;
}

diesel::table! {
    all_logs (id) {
        log_name -> Nullable<Text>,
        session_id -> Nullable<Uuid>,
        logger_used -> Uuid,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    dive_sessions (id) {
        start_time -> Timestamp,
        end_time -> Timestamp,
        session_name -> Nullable<Text>,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    dives (id) {
        discipline_type -> Nullable<Text>,
        depth -> Nullable<Float8>,
        distance -> Nullable<Float8>,
        dive_time -> Nullable<Int8>,
        dive_name -> Nullable<Text>,
        session_id -> Uuid,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PredefinedInputTypes;

    log_entries (id) {
        item_order -> Nullable<Int4>,
        category_type -> Text,
        input_type -> PredefinedInputTypes,
        input_value -> Nullable<Text>,
        log_id -> Uuid,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PredefinedInputTypes;

    logger_entries (id) {
        item_order -> Nullable<Int4>,
        field_name -> Text,
        category_name -> Text,
        input_type -> PredefinedInputTypes,
        logger_id -> Uuid,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    loggers (id) {
        logger_name -> Text,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        username -> Text,
        hashed_password -> Text,
        password_salt -> Bytea,
        email -> Text,
        last_login -> Timestamp,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::joinable!(all_logs -> dive_sessions (session_id));
diesel::joinable!(all_logs -> loggers (logger_used));
diesel::joinable!(all_logs -> users (user_id));
diesel::joinable!(dive_sessions -> users (user_id));
diesel::joinable!(dives -> dive_sessions (session_id));
diesel::joinable!(dives -> users (user_id));
diesel::joinable!(log_entries -> all_logs (log_id));
diesel::joinable!(log_entries -> users (user_id));
diesel::joinable!(logger_entries -> loggers (logger_id));
diesel::joinable!(logger_entries -> users (user_id));
diesel::joinable!(loggers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    all_logs,
    dive_sessions,
    dives,
    log_entries,
    logger_entries,
    loggers,
    users,
);
