use crate::data::UserQueryData;
use crate::diesel::ExpressionMethods;

use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_user(conn: &mut PgConnection, query_id: Uuid) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(user_id.eq(&query_id))
        .first::<UserQueryData>(conn)
        .expect("error loading person that was just inserted");

    Ok(user)
}
