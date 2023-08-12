// @generated automatically by Diesel CLI.

diesel::table! {
    apnea_sessions (id) {
        report_data -> Jsonb,
        form_id -> Uuid,
        original_form_id -> Nullable<Uuid>,
        previous_session_id -> Nullable<Uuid>,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_active -> Bool,
        archived_at -> Nullable<Timestamptz>,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_active -> Bool,
        archived_at -> Nullable<Timestamptz>,
        archived_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    unique_apneas (id) {
        activity_data -> Jsonb,
        session_id -> Uuid,
        user_id -> Uuid,
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_active -> Bool,
        archived_at -> Nullable<Timestamptz>,
        archived_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        username -> Text,
        hashed_password -> Text,
        password_salt -> Bytea,
        email -> Text,
        last_login -> Timestamptz,
        is_email_verified -> Bool,
        verified_date -> Nullable<Timestamptz>,
        verification_code -> Nullable<Text>,
        verification_code_expiry -> Nullable<Timestamptz>,
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_active -> Bool,
        archived_at -> Nullable<Timestamptz>,
        archived_by -> Nullable<Uuid>,
    }
}

diesel::joinable!(apnea_sessions -> users (user_id));
diesel::joinable!(forms -> users (user_id));
diesel::joinable!(unique_apneas -> apnea_sessions (session_id));
diesel::joinable!(unique_apneas -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    apnea_sessions,
    forms,
    unique_apneas,
    users,
);
