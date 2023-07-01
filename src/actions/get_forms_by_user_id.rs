use crate::actions::get_form_fields_by_form;
use crate::apnea_forms::form_helper::{FormStructure, FormStructureOutput};
use crate::dto::form_dto::FormOutput;
use crate::dto::form_field_dto::FormField;
use crate::dto::query_dto::QueryParams;
use crate::errors::{BigError, DieselQuerySnafu};
use crate::{diesel::ExpressionMethods, dto::form_dto::Form};
use diesel::{BoolExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use snafu::prelude::*;
use snafu::ResultExt;
use uuid::Uuid;

pub fn get_forms_by_user_id(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    _db_query_ob: Option<QueryParams>,
) -> Result<Vec<FormOutput>, BigError> {
    use crate::schema::forms::dsl::{forms, user_id as forms_user_id};

    let my_forms = forms
        .filter(forms_user_id.eq(&input_user_id))
        .get_results::<Form>(conn)
        .context(DieselQuerySnafu)?;

    use crate::schema::form_fields::dsl::{form_fields, form_id, user_id};

    let form_ids = my_forms.iter().map(|f| f.id).collect::<Vec<Uuid>>();

    let my_form_fields = form_fields
        .filter(form_id.eq_any(form_ids).and(user_id.eq(&input_user_id)))
        .get_results::<FormField>(conn)
        .context(DieselQuerySnafu)?;

    FormStructure::construct_from_forms(my_forms, my_form_fields)
}
