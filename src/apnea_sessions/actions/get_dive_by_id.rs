use crate::{apnea_sessions::dto::dive_dto::Dive, diesel::ExpressionMethods};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_by_id(conn: &mut PgConnection, input_dive_id: Uuid) -> diesel::QueryResult<Dive> {
    use crate::schema::dives::dsl::{dives, id as dive_id};

    dives
        .filter(dive_id.eq(&input_dive_id))
        .get_result::<Dive>(conn)
}
