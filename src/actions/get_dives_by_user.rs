use crate::{
    diesel::ExpressionMethods,
    dto::{
        db_query_dto::DBQueryObject,
        dive_dto::{DiveQueryData, DiveQueryInput},
    },
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dives_by_user(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    dive_query_input: Option<DiveQueryInput>,
    db_query_ob: Option<DBQueryObject>,
) -> diesel::QueryResult<Vec<DiveQueryData>> {
    use crate::schema::dives::dsl::*;

    let dives_output = dives
        .filter(user_id.eq(&input_user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(10) as i64)
        .get_results::<DiveQueryData>(conn)
        .expect("error loading dives");

    Ok(dives_output)
}