use crate::data::UserQueryData;
use crate::diesel::ExpressionMethods;

use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_user_with_email(
    conn: &mut PgConnection,
    query_email: String,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(email.eq(&query_email))
        .first::<UserQueryData>(conn)
        .expect("error loading person that was just inserted");

    Ok(user)
}
