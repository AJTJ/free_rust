use crate::apnea_forms::form_helper::{FormStructure, FormStructureOutput};
use crate::dto::completed_form_field_dto::CompletedFormField;
use crate::dto::query_dto::QueryParams;
use crate::dto::report_dto::CompletedForm;
use crate::errors::{BigError, DieselQuerySnafu};
use crate::{
    diesel::{BoolExpressionMethods, ExpressionMethods},
    errors::ChronoParseSnafu,
};
use async_graphql::connection::{Connection, Edge, EmptyFields};
use chrono::NaiveDateTime;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;
use uuid::Uuid;

pub fn get_completed_forms_by_user_id(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    query_params: QueryParams,
) -> Result<Connection<String, FormStructureOutput>, BigError> {
    use crate::schema::completed_forms::dsl::{
        completed_forms, created_at, user_id as forms_user_id,
    };

    let mut query = completed_forms
        .filter(forms_user_id.eq(&input_user_id))
        .into_boxed();

    if let Some(after) = &query_params.after {
        let after = after.parse::<NaiveDateTime>().context(ChronoParseSnafu)?;
        query = query.filter(created_at.gt(after))
    }

    let my_completed_forms: Vec<CompletedForm> = query
        .limit(query_params.first.and_then(|q| Some(q)).unwrap_or(10) as i64)
        .get_results::<CompletedForm>(conn)
        .context(DieselQuerySnafu)?;

    let completed_form_ids = my_completed_forms
        .iter()
        .map(|f| f.id)
        .collect::<Vec<Uuid>>();

    use crate::schema::completed_form_fields::dsl::{
        completed_form_fields, completed_form_id, user_id,
    };

    let desired_count = query_params.first.unwrap_or(10);
    let my_form_fields = completed_form_fields
        .filter(
            completed_form_id
                .eq_any(completed_form_ids)
                .and(user_id.eq(&input_user_id)),
        )
        .get_results::<CompletedFormField>(conn)
        .context(DieselQuerySnafu)?;

    let res = FormStructure::construct_from_completed_forms(&my_completed_forms, &my_form_fields)?;

    let mut connection = Connection::new(query_params.after.is_some(), res.len() > desired_count);
    connection.edges.extend(
        res.into_iter()
            .zip(my_completed_forms.iter())
            .take(desired_count)
            .map(|(fs, complete_form)| {
                Edge::with_additional_fields(complete_form.created_at.to_string(), fs, EmptyFields)
            }),
    );

    Ok(connection)
}
