use chrono::NaiveDateTime;

struct UserCreation {
    // id: uuid,
    username: String,
    hashed_password: String,
    email: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
