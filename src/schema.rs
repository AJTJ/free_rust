// @generated automatically by Diesel CLI.

diesel::table! {
    apnea_sessions (id) {
        start_time -> Timestamp,
        end_time -> Timestamp,
        session_name -> Nullable<Text>,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        archived_at -> Nullable<Timestamp>,
        archived_by -> Nullable<Uuid>,
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
        archived_at -> Nullable<Timestamp>,
        archived_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    forms (id) {
        form_name -> Text,
        form_data -> Jsonb,
        user_id -> Uuid,
        original_form_id -> Nullable<Uuid>,
        previous_form_id -> Nullable<Uuid>,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        archived_at -> Nullable<Timestamp>,
        archived_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    reports (id) {
        report_data -> Jsonb,
        form_id -> Uuid,
        original_form_id -> Nullable<Uuid>,
        previous_report_id -> Nullable<Uuid>,
        session_id -> Uuid,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
        archived_at -> Nullable<Timestamp>,
        archived_by -> Nullable<Uuid>,
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
        archived_at -> Nullable<Timestamp>,
        archived_by -> Nullable<Uuid>,
    }
}

diesel::joinable!(apnea_sessions -> users (user_id));
diesel::joinable!(dives -> apnea_sessions (session_id));
diesel::joinable!(dives -> users (user_id));
diesel::joinable!(forms -> users (user_id));
diesel::joinable!(reports -> apnea_sessions (session_id));
diesel::joinable!(reports -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    apnea_sessions,
    dives,
    forms,
    reports,
    users,
);
