use crate::diesel::ExpressionMethods;
use crate::dto::{db_query_dto::DBQueryObject, loggers_dto::LoggerEntryQueryData};
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

pub fn get_logger_entries_by_user(
    conn: &mut PgConnection,
    input_logger_id: &Uuid,
    db_query_ob: Option<DBQueryObject>,
) -> QueryResult<Vec<LoggerEntryQueryData>> {
    use crate::schema::logger_entries::dsl::{logger_entries, logger_id};

    logger_entries
        .filter(logger_id.eq(&input_logger_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(10) as i64)
        .get_results::<LoggerEntryQueryData>(conn)
}
