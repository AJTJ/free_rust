use crate::{diesel::ExpressionMethods, dto::dive_dto::DiveQuery};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_by_id(
    conn: &mut PgConnection,
    input_dive_id: Uuid,
) -> diesel::QueryResult<DiveQuery> {
    use crate::schema::dives::dsl::{dives, id as dive_id};

    dives
        .filter(dive_id.eq(&input_dive_id))
        .get_result::<DiveQuery>(conn)
}
