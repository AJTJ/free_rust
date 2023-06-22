// @generated automatically by Diesel CLI.

diesel::table! {
    completed_form_fields (id) {
        item_order -> Nullable<Int4>,
        field_name -> Text,
        field_value -> Nullable<Text>,
        category_name -> Text,
        field_value_type -> Text,
        completed_form_id -> Uuid,
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
    completed_forms (id) {
        completed_form_name -> Nullable<Text>,
        template_version -> Array<Nullable<Int4>>,
        form_id -> Uuid,
        original_form_id -> Nullable<Uuid>,
        previous_completed_form_id -> Nullable<Uuid>,
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
    dive_sessions (id) {
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
    form_fields (id) {
        item_order -> Nullable<Int4>,
        field_name -> Text,
        field_value -> Nullable<Text>,
        category_name -> Text,
        field_value_type -> Text,
        form_id -> Uuid,
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
        template_version -> Array<Nullable<Int4>>,
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

diesel::joinable!(completed_form_fields -> completed_forms (completed_form_id));
diesel::joinable!(completed_form_fields -> users (user_id));
diesel::joinable!(completed_forms -> dive_sessions (session_id));
diesel::joinable!(completed_forms -> users (user_id));
diesel::joinable!(dive_sessions -> users (user_id));
diesel::joinable!(dives -> dive_sessions (session_id));
diesel::joinable!(dives -> users (user_id));
diesel::joinable!(form_fields -> forms (form_id));
diesel::joinable!(form_fields -> users (user_id));
diesel::joinable!(forms -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    completed_form_fields,
    completed_forms,
    dive_sessions,
    dives,
    form_fields,
    forms,
    users,
);
