use crate::{
    apnea_forms::{
        dto::report_dto::{Report, ReportsRetrievalData},
        helpers::FormResponse,
    },
    utility::{
        errors::{BigError, ChronoParseSnafu, DieselQuerySnafu},
        gql::query_dto::QueryParams,
    },
};
use async_graphql::connection::{Connection, Edge, EmptyFields};
use chrono::{DateTime, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl,
};
use snafu::ResultExt;

pub fn get_reports(
    conn: &mut PgConnection,
    report_retrieval: ReportsRetrievalData,
    query_params: QueryParams,
) -> Result<Connection<String, Report>, BigError> {
    use crate::schema::reports::dsl::{
        created_at, id as report_id, reports, user_id as forms_user_id,
    };

    let mut query = match report_retrieval {
        ReportsRetrievalData::UserId(input_user_id) => {
            reports.filter(forms_user_id.eq(input_user_id)).into_boxed()
        }
        ReportsRetrievalData::ReportIds(report_ids) => {
            reports.filter(report_id.eq_any(report_ids)).into_boxed()
        }
    };

    if let Some(after) = &query_params.after {
        let after = after.parse::<DateTime<Utc>>().context(ChronoParseSnafu)?;
        query = query.filter(created_at.gt(after))
    }

    let my_reports: Option<Vec<Report>> = query
        .limit(query_params.first.and_then(|q| Some(q)).unwrap_or(10) as i64)
        .get_results::<Report>(conn)
        .optional()
        .context(DieselQuerySnafu)?;

    let desired_count = query_params.first.unwrap_or(10);

    let mut connection = Connection::new(
        query_params.after.is_some(),
        my_reports.clone().unwrap_or_else(|| vec![]).len() > desired_count,
    );

    let output_vals = my_reports
        .unwrap_or_else(|| vec![])
        .into_iter()
        .map(|r| (r.created_at.to_string(), r))
        .collect::<Vec<(String, Report)>>();

    connection.edges.extend(
        output_vals
            .into_iter()
            .take(desired_count)
            .map(|(key, report)| Edge::with_additional_fields(key, report, EmptyFields)),
    );

    Ok(connection)
}
