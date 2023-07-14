use crate::apnea_forms::dto::form_dto::Form;
use crate::diesel::ExpressionMethods;
use crate::utility::errors::BigError;
use crate::utility::errors::DieselQuerySnafu;
use crate::utility::gql::query_dto::QueryParams;
use diesel::{BoolExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use snafu::prelude::*;
use snafu::ResultExt;
use uuid::Uuid;

pub fn get_forms_by_user(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    query_params: QueryParams,
) -> Result<Option<Vec<Form>>, BigError> {
    use crate::schema::forms::dsl::{forms, user_id as forms_user_id};

    let my_forms = forms
        .filter(forms_user_id.eq(&input_user_id))
        .get_results::<Form>(conn)
        .optional()
        .context(DieselQuerySnafu);

    my_forms
}
