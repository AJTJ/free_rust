use crate::{
    diesel::ExpressionMethods,
    dto::{
        db_query_dto::DBQueryObject,
        dive_session_dto::{DiveSessionQueryData, DiveSessionQueryInput},
    },
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_sessions_by_user(
    conn: &mut PgConnection,
    user_id: &Uuid,
    dive_session_query_input: Option<DiveSessionQueryInput>,
    db_query_ob: Option<DBQueryObject>,
) -> diesel::QueryResult<Vec<DiveSessionQueryData>> {
    use crate::schema::dive_sessions::dsl::*;

    let dive_sessions_output = dive_sessions
        .filter(user_id.eq(&user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(10) as i64)
        .get_results::<DiveSessionQueryData>(conn)
        .expect("error loading dive sessions");

    Ok(dive_sessions_output)
}
