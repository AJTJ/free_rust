use crate::{
    diesel::ExpressionMethods,
    dto::{db_query_dto::DBQueryObject, dive_session_dto::DiveSessionQueryData},
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_session_by_id(
    conn: &mut PgConnection,
    input_session_id: &Uuid,
    db_query_ob: Option<DBQueryObject>,
) -> diesel::QueryResult<DiveSessionQueryData> {
    use crate::schema::dive_sessions::dsl::{dive_sessions, unique_id as session_id};

    let dive_sessions_output = dive_sessions
        .filter(session_id.eq(&input_session_id))
        .limit(1)
        .get_result::<DiveSessionQueryData>(conn)
        .expect("error loading dive sessions");

    Ok(dive_sessions_output)
}
