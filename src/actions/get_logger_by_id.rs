use crate::{diesel::ExpressionMethods, dto::logger_dto::Logger};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_logger_by_id(
    conn: &mut PgConnection,
    logger_id_input: Uuid,
) -> diesel::QueryResult<Logger> {
    use crate::schema::loggers::dsl::*;

    loggers
        .filter(id.eq(&logger_id_input))
        .get_result::<Logger>(conn)
}
