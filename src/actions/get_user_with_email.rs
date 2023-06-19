use crate::diesel::ExpressionMethods;
use crate::dto::user_dto::UserQuery;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn get_user_with_email(
    conn: &mut PgConnection,
    query_email: String,
) -> diesel::QueryResult<UserQuery> {
    use crate::schema::users::dsl::*;

    users
        .filter(email.eq(&query_email))
        .first::<UserQuery>(conn)
}
