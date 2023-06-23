use crate::{
    diesel::ExpressionMethods,
    dto::{dive_session_dto::DiveSession, query_dto::QueryParams},
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_sessions_by_id(
    conn: &mut PgConnection,
    input_session_ids: &[Uuid],
) -> diesel::QueryResult<Vec<DiveSession>> {
    use crate::schema::dive_sessions::dsl::{created_at, dive_sessions, id as session_id};

    dive_sessions
        .filter(session_id.eq_any(&input_session_ids.to_vec()))
        .order(created_at)
        .get_results::<DiveSession>(conn)
}
