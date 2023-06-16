use crate::dto::db_query_dto::DBQueryObject;
use crate::{diesel::ExpressionMethods, dto::loggers_dto::LoggerQueryData};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_loggers_from_id(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    db_query_ob: Option<DBQueryObject>,
) -> diesel::QueryResult<Vec<LoggerQueryData>> {
    use crate::schema::loggers::dsl::{loggers, user_id};

    loggers
        .filter(user_id.eq(&input_user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(100) as i64)
        .get_results::<LoggerQueryData>(conn)
}
