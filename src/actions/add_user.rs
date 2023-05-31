use crate::data::{UserCreationData, UserInputData};
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use chrono::Utc;
use diesel::{PgConnection, RunQueryDsl};
use uuid::Uuid;

pub fn add_user(
    conn: &mut PgConnection,
    user_data: UserInputData,
) -> diesel::QueryResult<UserCreationData> {
    use crate::schema::users::dsl::*;

    let current_stamp = Utc::now().naive_utc();

    let uuid = Uuid::new_v4();

    let new_user = UserCreationData {
        user_id: uuid.clone(),
        username: user_data.username,
        hashed_password: user_data.hashed_password,
        email: user_data.email,
        created_at: current_stamp,
        updated_at: current_stamp,
    };

    diesel::insert_into(users).values(&new_user).execute(conn);

    let user = users
        .filter(user_id.eq(&uuid))
        .first::<UserCreationData>(conn)
        .expect("thang");

    Ok(user)
}
