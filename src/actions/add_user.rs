use crate::data::{UserCreationData, UserInputData, UserQueryData};
use crate::diesel::ExpressionMethods;

use chrono::Utc;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn add_user(
    conn: &mut PgConnection,
    user_data: UserInputData,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let current_stamp = Utc::now().naive_utc();

    let uuid = Uuid::new_v4();

    let new_user = UserCreationData {
        username: user_data.username,
        user_id: uuid,
        hashed_password: user_data.hashed_password,
        email: user_data.email,
        created_at: current_stamp,
        updated_at: current_stamp,
    };

    // TODO: need to return a useful error upon failed insert
    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let user = users
        .filter(user_id.eq(&uuid))
        .first::<UserQueryData>(conn)
        .expect("error loading person that was just inserted");

    Ok(user)
}

// let user = web::block(move || {
//     let pool_ctx = ctx.data::<DbPool>().unwrap();
//     let conn = pool_ctx.get().unwrap();
//     diesel::insert_into(users)
//         .values(&new_user)
//         .execute(&conn)
//         .unwrap();
//     users
//         .filter(user_id.eq(&uuid))
//         .first::<UserQueryData>(&conn)
//         .expect("error loading person that was just inserted")
// })
// .await
// .unwrap();

// Ok(user)
