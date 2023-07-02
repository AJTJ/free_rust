use crate::{
    apnea_sessions::dto::apnea_session_dto::ApneaSession, diesel::ExpressionMethods,
    utility::gql::query_dto::QueryParams,
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_apnea_session(
    conn: &mut PgConnection,
    input_session_id: &Uuid,
    db_query_ob: Option<QueryParams>,
) -> diesel::QueryResult<ApneaSession> {
    use crate::schema::apnea_sessions::dsl::{apnea_sessions, created_at, id as session_id};

    apnea_sessions
        .filter(session_id.eq(&input_session_id))
        .order(created_at)
        .get_result::<ApneaSession>(conn)
}
