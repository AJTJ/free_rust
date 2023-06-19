use crate::{
    diesel::ExpressionMethods,
    dto::{
        db_query_dto::DBQueryParams,
        dive_session_dto::{DiveSessionQuery, DiveSessionQueryParams},
    },
};
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

pub fn get_dive_sessions_by_user(
    conn: &mut PgConnection,
    input_user_id: &Uuid,
    dive_session_query_input: Option<DiveSessionQueryParams>,
    db_query_ob: Option<DBQueryParams>,
) -> QueryResult<Vec<DiveSessionQuery>> {
    use crate::schema::dive_sessions::dsl::{dive_sessions, user_id};

    dive_sessions
        .filter(user_id.eq(&input_user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(10) as i64)
        .get_results::<DiveSessionQuery>(conn)
}
