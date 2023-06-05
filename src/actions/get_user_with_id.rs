use crate::{diesel::ExpressionMethods, dto::user_auth_dto::UserQueryData};

use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_user_with_id(
    conn: &mut PgConnection,
    query_id: Uuid,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(user_id.eq(&query_id))
        .first::<UserQueryData>(conn)
        .expect("error loading person that was just inserted");

    Ok(user)
}
