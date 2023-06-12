use crate::{diesel::ExpressionMethods, dto::dive_dto::DiveQueryData};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_by_id(
    conn: &mut PgConnection,
    input_dive_id: Uuid,
) -> diesel::QueryResult<DiveQueryData> {
    use crate::schema::dives::dsl::{dives, unique_id as dive_id};

    let dive_output = dives
        .filter(dive_id.eq(&input_dive_id))
        .get_result::<DiveQueryData>(conn)
        .expect("error loading dives");

    Ok(dive_output)
}
