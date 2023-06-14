// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "predefined_field_names"))]
    pub struct PredefinedFieldNames;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "predefined_input_types"))]
    pub struct PredefinedInputTypes;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "predefined_session_categories"))]
    pub struct PredefinedSessionCategories;
}

diesel::table! {
    all_logs (id) {
        session_id -> Nullable<Uuid>,
        user_id -> Uuid,
        logger_used -> Uuid,
        id -> Int4,
        unique_id -> Uuid,
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
        id -> Int4,
        unique_id -> Uuid,
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
        id -> Int4,
        unique_id -> Uuid,
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
    use super::sql_types::PredefinedSessionCategories;

    log_input_entries (id) {
        item_order -> Nullable<Int4>,
        input_value_type -> PredefinedInputTypes,
        category_type -> PredefinedSessionCategories,
        input_enum -> Nullable<Text>,
        input_integer -> Nullable<Int4>,
        input_interval -> Nullable<Interval>,
        input_timestamp -> Nullable<Timestamp>,
        input_text -> Nullable<Text>,
        log_id -> Uuid,
        user_id -> Uuid,
        id -> Int4,
        unique_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PredefinedSessionCategories;

    logger_category_entries (id) {
        item_order -> Nullable<Int4>,
        logger_category_type -> PredefinedSessionCategories,
        logger_id -> Uuid,
        user_id -> Uuid,
        id -> Int4,
        unique_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PredefinedFieldNames;
    use super::sql_types::PredefinedSessionCategories;
    use super::sql_types::PredefinedInputTypes;

    logger_input_entries (id) {
        item_order -> Nullable<Int4>,
        field_name -> PredefinedFieldNames,
        category_type -> PredefinedSessionCategories,
        input_type -> PredefinedInputTypes,
        logger_id -> Uuid,
        user_id -> Uuid,
        id -> Int4,
        unique_id -> Uuid,
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
        id -> Int4,
        unique_id -> Uuid,
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
        id -> Int4,
        unique_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    valid_enum_inputs (input_name) {
        input_name -> Text,
    }
}

diesel::joinable!(log_input_entries -> valid_enum_inputs (input_enum));

diesel::allow_tables_to_appear_in_same_query!(
    all_logs,
    dive_sessions,
    dives,
    log_input_entries,
    logger_category_entries,
    logger_input_entries,
    loggers,
    users,
    valid_enum_inputs,
);
