use crate::diesel::ExpressionMethods;
use crate::dto::{db_query_dto::DBQueryObject, loggers_dto::LoggerEntryData};
use crate::errors::BigError;
use diesel::{BoolExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

pub fn get_logger_entries_by_logger(
    conn: &mut PgConnection,
    input_logger_id: &Uuid,
    input_user_id: &Uuid,
    db_query_ob: Option<DBQueryObject>,
) -> QueryResult<Vec<LoggerEntryData>> {
    use crate::schema::logger_entries::dsl::{logger_entries, logger_id, user_id};

    logger_entries
        .filter(
            logger_id
                .eq(&input_logger_id)
                .and(user_id.eq(&input_user_id)),
        )
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(10) as i64)
        .get_results::<LoggerEntryData>(conn)
}
