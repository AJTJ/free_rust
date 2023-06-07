use crate::{
    diesel::ExpressionMethods,
    dto::{
        db_query_dto::DBQueryObject,
        dive_session_dto::{DiveSessionQueryData, DiveSessionQueryInput},
    },
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn get_dives(
    conn: &mut PgConnection,
    dive_session_query_input: DiveSessionQueryInput,
    db_query_ob: DBQueryObject,
) -> diesel::QueryResult<Vec<DiveSessionQueryData>> {
    use crate::schema::dive_sessions::dsl::*;

    let dive_sessions_output = dive_sessions
        .filter(user_id.eq(&dive_session_query_input.user_id))
        .limit(db_query_ob.limit.unwrap_or(10) as i64)
        .get_results::<DiveSessionQueryData>(conn)
        .expect("error loading dive sessions");

    Ok(dive_sessions_output)
}
