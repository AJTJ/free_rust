use crate::{
    apnea_sessions::dto::apnea_session_dto::{ApneaSession, ApneaSessionRetrievalData},
    diesel::ExpressionMethods,
    utility::{
        errors::{BigError, ChronoParseSnafu, DieselQuerySnafu},
        gql::query_dto::QueryParams,
    },
};
use async_graphql::connection::{Connection, Edge, EmptyFields};
use chrono::{DateTime, Utc};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;

pub fn get_apnea_sessions_paginated(
    conn: &mut PgConnection,
    retrieval_method: ApneaSessionRetrievalData,
    query_params: QueryParams,
) -> Result<Connection<String, ApneaSession>, BigError> {
    use crate::schema::apnea_sessions::dsl::{
        apnea_sessions, created_at, id as session_id, user_id,
    };

    let mut query = match retrieval_method {
        ApneaSessionRetrievalData::Sessions(s) => {
            apnea_sessions.filter(session_id.eq_any(s)).into_boxed()
        }
        ApneaSessionRetrievalData::User(input_user_id) => apnea_sessions
            .filter(user_id.eq(input_user_id))
            .into_boxed(),
    };

    if let Some(after) = &query_params.after {
        query =
            query.filter(created_at.gt(after.parse::<DateTime<Utc>>().context(ChronoParseSnafu)?))
    }

    let desired_count = query_params.first.unwrap_or(10);
    let res: Vec<ApneaSession> = query
        .limit(desired_count as i64)
        .get_results::<ApneaSession>(conn)
        .context(DieselQuerySnafu)?;

    let mut connection = Connection::new(query_params.after.is_some(), res.len() > desired_count);
    connection
        .edges
        .extend(res.into_iter().take(desired_count).map(|session| {
            Edge::with_additional_fields(session.created_at.to_string(), session, EmptyFields)
        }));

    Ok(connection)
}

// apnea_sessions
// .filter(session_id.eq_any(&s))
// .order(created_at)
// .get_results::<DiveSession>(conn)
// .context(DieselQuerySnafu),
