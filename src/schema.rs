// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "custom_type"))]
    pub struct CustomType;
}

diesel::table! {
    all_logs (id) {
        session_id -> Nullable<Uuid>,
        dive_id -> Nullable<Uuid>,
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
    custom_enum_categories (id) {
        category_name -> Nullable<Text>,
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
    custom_enum_variants (id) {
        variant_name -> Nullable<Text>,
        custom_enum_category_id -> Uuid,
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
    log_category_entries (id) {
        item_order -> Nullable<Int4>,
        logger_category_type_id -> Uuid,
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
    use super::sql_types::CustomType;

    log_input_entries (id) {
        item_order -> Nullable<Int4>,
        input_type_used -> CustomType,
        input_float -> Nullable<Float8>,
        input_integer -> Nullable<Int4>,
        custom_enum_category_id -> Nullable<Uuid>,
        custom_enum_variant_id -> Nullable<Uuid>,
        log_category_entry_id -> Uuid,
        logger_input_type_id -> Uuid,
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
    logger_category_entries (id) {
        item_order -> Nullable<Int4>,
        logger_category_type_id -> Uuid,
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
    logger_category_types (id) {
        logger_category_name -> Text,
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
    logger_input_entries (id) {
        item_order -> Nullable<Int4>,
        logger_input_type_id -> Uuid,
        logger_category_entry_id -> Uuid,
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
    use super::sql_types::CustomType;

    logger_input_types (id) {
        logger_input_name -> Text,
        input_value_type -> CustomType,
        custom_enum_category_id -> Nullable<Uuid>,
        logger_category_id -> Uuid,
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

diesel::allow_tables_to_appear_in_same_query!(
    all_logs,
    custom_enum_categories,
    custom_enum_variants,
    dive_sessions,
    dives,
    log_category_entries,
    log_input_entries,
    logger_category_entries,
    logger_category_types,
    logger_input_entries,
    logger_input_types,
    loggers,
    users,
);
