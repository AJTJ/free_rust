use crate::{
    diesel::ExpressionMethods,
    dto::{
        db_query_dto::DBQueryObject,
        dive_dto::{DiveQueryData, DiveQueryInput},
    },
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn get_dives(
    conn: &mut PgConnection,
    dive_query_input: DiveQueryInput,
    db_query_ob: DBQueryObject,
) -> diesel::QueryResult<Vec<DiveQueryData>> {
    use crate::schema::dives::dsl::*;

    let dive_sessions_output = dives
        .filter(user_id.eq(&dive_query_input.user_id))
        .limit(db_query_ob.limit.unwrap_or(10) as i64)
        .get_results::<DiveQueryData>(conn)
        .expect("error loading dive sessions");

    Ok(dive_sessions_output)
}
