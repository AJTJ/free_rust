use crate::apnea_forms::dto::form_dto::Form;
use crate::diesel::ExpressionMethods;
use crate::utility::errors::BigError;
use crate::utility::errors::DieselQuerySnafu;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;
use tracing::{event, Level};
use uuid::Uuid;

pub fn get_forms_by_id(
    conn: &mut PgConnection,
    input_form_ids: Vec<Uuid>,
) -> Result<Option<Vec<Form>>, BigError> {
    event!(Level::DEBUG, "in get_forms_by_id");
    use crate::schema::forms::dsl::{forms, id as form_id};

    let my_forms = forms
        .filter(form_id.eq_any(input_form_ids))
        .get_results::<Form>(conn)
        .optional()
        .context(DieselQuerySnafu);

    my_forms
}
