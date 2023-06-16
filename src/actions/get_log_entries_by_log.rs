use crate::diesel::ExpressionMethods;
use crate::dto::db_query_dto::DBQueryObject;
use crate::dto::log_dto::LogEntryData;
use crate::errors::BigError;
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

pub fn get_log_entries_by_log(
    conn: &mut PgConnection,
    input_log_id: &Uuid,
    db_query_ob: Option<DBQueryObject>,
) -> QueryResult<Vec<LogEntryData>> {
    use crate::schema::log_entries::dsl::{log_entries, log_id};

    log_entries
        .filter(log_id.eq(&input_log_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(1000) as i64)
        .get_results::<LogEntryData>(conn)
}
