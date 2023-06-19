use crate::diesel::ExpressionMethods;
use crate::dto::db_query_dto::DBQueryParams;
use crate::dto::log_dto::Log;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_logs_from_user_id(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    db_query_ob: Option<DBQueryParams>,
) -> diesel::QueryResult<Vec<Log>> {
    use crate::schema::all_logs::dsl::{all_logs, user_id};

    all_logs
        .filter(user_id.eq(&input_user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(100) as i64)
        .get_results::<Log>(conn)
}
