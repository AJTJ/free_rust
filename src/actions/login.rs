use crate::data::UserQueryData;
use crate::diesel::ExpressionMethods;

use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn login(
    conn: &mut PgConnection,
    inc_email: String,
    password: String,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let maybe_user = users
        .filter(email.eq(&inc_email))
        .first::<UserQueryData>(conn)
        .expect("user not found");

    Ok(maybe_user)
}
