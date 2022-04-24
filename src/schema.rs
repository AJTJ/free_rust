table! {
    dive_sessions (id) {
        id -> Int4,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        session_name -> Nullable<Text>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    dives (id) {
        id -> Int4,
        discipline_type -> Nullable<Discipline>,
        depth -> Nullable<Float8>,
        distance -> Nullable<Float8>,
        dive_time -> Nullable<Time>,
        dive_name -> Nullable<Text>,
        dive_session -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(dive_sessions -> users (user_id));
joinable!(dives -> dive_sessions (dive_session));
joinable!(dives -> users (user_id));

allow_tables_to_appear_in_same_query!(dive_sessions, dives, users,);
