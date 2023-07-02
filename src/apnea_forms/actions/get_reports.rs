use crate::{
    apnea_forms::{dto::report_dto::Report, helpers::FormOutput},
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
use uuid::Uuid;

pub fn get_reports(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    query_params: QueryParams,
) -> Result<Connection<String, FormOutput>, BigError> {
    use crate::schema::reports::dsl::{created_at, reports, user_id as forms_user_id};

    let mut query = reports
        .filter(forms_user_id.eq(&input_user_id))
        .into_boxed();

    if let Some(after) = &query_params.after {
        let after = after.parse::<NaiveDateTime>().context(ChronoParseSnafu)?;
        query = query.filter(created_at.gt(after))
    }

    let my_reports: Vec<Report> = query
        .limit(query_params.first.and_then(|q| Some(q)).unwrap_or(10) as i64)
        .get_results::<Report>(conn)
        .context(DieselQuerySnafu)?;

    let desired_count = query_params.first.unwrap_or(10);

    let mut connection = Connection::new(
        query_params.after.is_some(),
        my_reports.len() > desired_count,
    );

    let output_vals = my_reports
        .into_iter()
        .map(|r| (r.created_at.to_string(), r.report_data))
        .collect::<Vec<(String, FormOutput)>>();

    connection.edges.extend(
        output_vals
            .into_iter()
            .take(desired_count)
            .map(|(key, report)| Edge::with_additional_fields(key, report, EmptyFields)),
    );

    Ok(connection)
}
