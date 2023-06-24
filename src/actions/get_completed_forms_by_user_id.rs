use crate::dto::completed_form_dto::CompletedForm;
use crate::dto::query_dto::QueryParams;
use crate::errors::{BigError, DieselQuerySnafu};
use crate::{diesel::ExpressionMethods, errors::ChronoParseSnafu};
use chrono::NaiveDateTime;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;
use uuid::Uuid;

pub fn get_completed_forms_by_user_id(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    query_params: QueryParams,
) -> Result<Vec<(String, CompletedForm)>, BigError> {
    use crate::schema::completed_forms::dsl::{completed_forms, created_at, user_id};

    let mut query = completed_forms
        .filter(user_id.eq(&input_user_id))
        .into_boxed();

    if let Some(after) = query_params.after {
        let after = after.parse::<NaiveDateTime>().context(ChronoParseSnafu)?;
        query = query.filter(created_at.gt(after))
    }

    let query_result: Vec<CompletedForm> = query
        .limit(query_params.first.and_then(|q| Some(q)).unwrap_or(10) as i64)
        .get_results::<CompletedForm>(conn)
        .context(DieselQuerySnafu)?;

    Ok(query_result
        .into_iter()
        .map(|d| (d.id.to_string(), d))
        .collect::<Vec<(String, CompletedForm)>>())
}
