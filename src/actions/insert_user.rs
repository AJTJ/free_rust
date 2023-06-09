use crate::auth_data::UniversalIdType;
use crate::diesel::ExpressionMethods;
use crate::dto::user_auth_dto::{UserCreationData, UserInputData, UserQueryData};
use argon2::{self, Config};

use chrono::Utc;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use rand::Rng;
use uuid::Uuid;

pub fn insert_user(
    conn: &mut PgConnection,
    user_data: UserInputData,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let current_stamp = Utc::now().naive_utc();

    let uuid = Uuid::new_v4();

    // PW + HASHING
    let salt_gen: UniversalIdType = rand::thread_rng().gen::<UniversalIdType>();
    let hashed_pw =
        argon2::hash_encoded(user_data.password.as_bytes(), &salt_gen, &Config::default()).unwrap();

    let new_user = UserCreationData {
        username: user_data.username,
        user_id: uuid,
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
        .execute(conn)
        .expect("diesel insert new user error");

    let user = users
        .filter(user_id.eq(&uuid))
        .first::<UserQueryData>(conn)
        .expect("error loading person that was just inserted");

    Ok(user)
}
