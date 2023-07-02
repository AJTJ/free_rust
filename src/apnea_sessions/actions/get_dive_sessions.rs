use crate::{
    apnea_sessions::dto::dive_session_dto::{
        ApnesSessionRetrievalData, DiveSession, DiveSessionFilter,
    },
    diesel::ExpressionMethods,
    utility::{
        errors::{BigError, ChronoParseSnafu, DieselQuerySnafu},
        gql::query_dto::QueryParams,
    },
};
use async_graphql::connection::{Connection, Edge, EmptyFields};
use chrono::NaiveDateTime;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;

pub fn get_dive_sessions(
    conn: &mut PgConnection,
    retrieval_method: ApnesSessionRetrievalData,
    dive_session_filter: Option<DiveSessionFilter>,
    query_params: QueryParams,
) -> Result<Connection<String, DiveSession>, BigError> {
    use crate::schema::dive_sessions::dsl::{created_at, dive_sessions, id as session_id, user_id};

    let mut query = match retrieval_method {
        ApnesSessionRetrievalData::Sessions(s) => {
            dive_sessions.filter(session_id.eq_any(s)).into_boxed()
        }
        ApnesSessionRetrievalData::User(input_user_id) => {
            dive_sessions.filter(user_id.eq(input_user_id)).into_boxed()
        }
    };

    if let Some(after) = &query_params.after {
        let after = after.parse::<NaiveDateTime>().context(ChronoParseSnafu)?;
        query = query.filter(created_at.gt(after))
    }

    let desired_count = query_params.first.unwrap_or(10);
    let res: Vec<DiveSession> = query
        .limit(desired_count as i64)
        .get_results::<DiveSession>(conn)
        .context(DieselQuerySnafu)?;

    let mut connection = Connection::new(query_params.after.is_some(), res.len() > desired_count);
    connection
        .edges
        .extend(res.into_iter().take(desired_count).map(|session| {
            Edge::with_additional_fields(session.created_at.to_string(), session, EmptyFields)
        }));

    Ok(connection)
}

// dive_sessions
// .filter(session_id.eq_any(&s))
// .order(created_at)
// .get_results::<DiveSession>(conn)
// .context(DieselQuerySnafu),
