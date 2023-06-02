use crate::data::UserQueryData;
use crate::diesel::ExpressionMethods;

use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn login(
    conn: &mut PgConnection,
    email: String,
    password: String,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(email.eq(&email))
        .first::<UserQueryData>(conn)
        .expect("user not found");

    Ok(user)
}
