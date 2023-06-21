use crate::{diesel::ExpressionMethods, dto::log_dto::Log};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_log_by_id(conn: &mut PgConnection, log_id_input: Uuid) -> diesel::QueryResult<Log> {
    use crate::schema::all_logs::dsl::*;

    all_logs
        .filter(id.eq(&log_id_input))
        .get_result::<Log>(conn)
}
