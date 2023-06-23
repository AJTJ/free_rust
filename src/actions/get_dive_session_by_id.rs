use crate::{
    diesel::ExpressionMethods,
    dto::{dive_session_dto::DiveSession, query_dto::QueryParams},
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_session_by_id(
    conn: &mut PgConnection,
    input_session_id: &Uuid,
    db_query_ob: Option<QueryParams>,
) -> diesel::QueryResult<DiveSession> {
    use crate::schema::dive_sessions::dsl::{created_at, dive_sessions, id as session_id};

    let q = dive_sessions
        .filter(session_id.eq(&input_session_id))
        .order(created_at)
        .get_result::<DiveSession>(conn);

    q
}
