use crate::{
    apnea_forms::{dto::report_dto::Report, formV1::form::FormOutputV1, helpers::AllFormsOutput},
    diesel::{BoolExpressionMethods, ExpressionMethods},
    utility::{
        errors::{BigError, ChronoParseSnafu, DieselQuerySnafu, SerdeParseSnafu},
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
) -> Result<Connection<String, AllFormsOutput>, BigError> {
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

    let report_ids = my_reports.iter().map(|f| f.id).collect::<Vec<Uuid>>();

    let desired_count = query_params.first.unwrap_or(10);

    let mut connection = Connection::new(
        query_params.after.is_some(),
        my_reports.len() > desired_count,
    );
    connection
        .edges
        .extend(my_reports.into_iter().take(desired_count).map(|report| {
            let form_output: Result<AllFormsOutput, BigError> = match report.report_version {
                1 => AllFormsOutput::V1(
                    serde_json::from_value::<FormOutputV1>(report.report_data)
                        .context(SerdeParseSnafu),
                ),
                _ => Err(BigError::VersionParsingError),
            };
            Edge::with_additional_fields(report.created_at.to_string(), form_output, EmptyFields)
        }));

    Ok(connection)
}
