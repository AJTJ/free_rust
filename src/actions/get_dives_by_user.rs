use crate::{
    diesel::ExpressionMethods,
    dto::{
        db_query_dto::DBQueryParams,
        dive_dto::{DiveQuery, DiveQueryInput},
    },
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dives_by_user(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    dive_query_input: Option<DiveQueryInput>,
    db_query_ob: Option<DBQueryParams>,
) -> diesel::QueryResult<Vec<DiveQuery>> {
    use crate::schema::dives::dsl::{dives, user_id};

    dives
        .filter(user_id.eq(&input_user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(10) as i64)
        .get_results::<DiveQuery>(conn)
}
