use crate::{apnea_sessions::dto::unique_apnea_dto::UniqueApnea, diesel::ExpressionMethods};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_unique_apnea(
    conn: &mut PgConnection,
    input_dive_id: Uuid,
) -> diesel::QueryResult<UniqueApnea> {
    use crate::schema::unique_apneas::dsl::{id as dive_id, unique_apneas};

    unique_apneas
        .filter(dive_id.eq(&input_dive_id))
        .get_result::<UniqueApnea>(conn)
}
