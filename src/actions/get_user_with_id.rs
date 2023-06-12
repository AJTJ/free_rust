use crate::{diesel::ExpressionMethods, dto::user_auth_dto::UserQueryData};

use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_user_with_id(
    conn: &mut PgConnection,
    query_id: &Uuid,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::{unique_id as user_id, users};

    users
        .filter(user_id.eq(&query_id))
        .first::<UserQueryData>(conn)
}
