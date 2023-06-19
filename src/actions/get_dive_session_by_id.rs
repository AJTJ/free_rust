use crate::{
    diesel::ExpressionMethods,
    dto::{db_query_dto::DBQueryParams, dive_session_dto::DiveSession},
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_session_by_id(
    conn: &mut PgConnection,
    input_session_id: &Uuid,
    db_query_ob: Option<DBQueryParams>,
) -> diesel::QueryResult<DiveSession> {
    use crate::schema::dive_sessions::dsl::{dive_sessions, id as session_id};

    dive_sessions
        .filter(session_id.eq(&input_session_id))
        .limit(1)
        .get_result::<DiveSession>(conn)
}
