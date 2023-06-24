use crate::{
    diesel::ExpressionMethods,
    dto::{
        dive_session_dto::{DiveSession, DiveSessionFilter},
        query_dto::QueryParams,
    },
    errors::{BigError, ChronoParseSnafu, DieselQuerySnafu},
};
use chrono::NaiveDateTime;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;
use uuid::Uuid;

pub fn get_dive_sessions_by_user(
    conn: &mut PgConnection,
    input_user_id: &Uuid,
    dive_session_filter: Option<DiveSessionFilter>,
    query_params: QueryParams,
) -> Result<Vec<(String, DiveSession)>, BigError> {
    use crate::schema::dive_sessions::dsl::{created_at, dive_sessions, user_id};

    let after = match query_params.after {
        Some(e) => Some(e.parse::<NaiveDateTime>().context(ChronoParseSnafu)),
        None => None,
    };

    let query = dive_sessions
        .filter(user_id.eq(&input_user_id))
        .into_boxed();

    if let Some(after) = query_params.after {
        let after = after.parse::<NaiveDateTime>().context(ChronoParseSnafu)?;
        query = query.filter(created_at.gt(after))
    }

    let res: Vec<DiveSession> = query
        .limit(query_params.first.and_then(|q| Some(q)).unwrap_or(10) as i64)
        .get_results::<DiveSession>(conn)
        .context(DieselQuerySnafu)?;

    Ok(res
        .into_iter()
        .map(|d| (d.id.to_string(), d))
        .collect::<Vec<(String, DiveSession)>>())
}
