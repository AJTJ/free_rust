use crate::auth::{
    dto::user_dto::{User, UserCreation, UserInput},
    utility::auth_data::UniversalIdType,
};
use crate::diesel::RunQueryDsl;
use argon2::{self, Config};
use chrono::Utc;
use diesel::PgConnection;
use rand::Rng;

pub fn insert_user(conn: &mut PgConnection, user_data: &UserInput) -> diesel::QueryResult<User> {
    use crate::schema::users::dsl::users;

    let current_stamp = Utc::now();

    // PW + HASHING
    let salt_gen: UniversalIdType = rand::thread_rng().gen::<UniversalIdType>();
    let hashed_pw =
        argon2::hash_encoded(user_data.password.as_bytes(), &salt_gen, &Config::default()).unwrap();

    let new_user = UserCreation {
        username: user_data.username,
        hashed_password: hashed_pw,
        password_salt: salt_gen.to_vec(),
        email: user_data.email,
        last_login: current_stamp,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
}
