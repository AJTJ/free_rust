use crate::{
    apnea_sessions::dto::dive_session_dto::DiveSession, diesel::ExpressionMethods,
    utility::gql::query_dto::QueryParams,
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_session(
    conn: &mut PgConnection,
    input_session_id: &Uuid,
    db_query_ob: Option<QueryParams>,
) -> diesel::QueryResult<DiveSession> {
    use crate::schema::dive_sessions::dsl::{created_at, dive_sessions, id as session_id};

    dive_sessions
        .filter(session_id.eq(&input_session_id))
        .order(created_at)
        .get_result::<DiveSession>(conn)
}
