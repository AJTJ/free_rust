use crate::{diesel::ExpressionMethods, dto::user_dto::UserQuery};

use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_user_with_id(
    conn: &mut PgConnection,
    query_id: &Uuid,
) -> diesel::QueryResult<UserQuery> {
    use crate::schema::users::dsl::{id as user_id, users};

    users.filter(user_id.eq(&query_id)).first::<UserQuery>(conn)
}
