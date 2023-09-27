use crate::apnea_forms::dto::form_dto::{Form, FormCreation, FormDetails};

use crate::apnea_forms::forms_interface::FormResponse;
use crate::graphql_schema::DbPool;
use crate::utility::errors::{BigError, SerdeSerializeSnafu};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{
    /* BoolExpressionMethods, ExpressionMethods, */
    OptionalExtension, /* PgConnection, QueryDsl, */
    RunQueryDsl,
};
use serde_json;
use snafu::ResultExt;
use uuid::Uuid;

pub async fn insert_form(
    ctx: &Context<'_>,
    form_request: FormDetails,
    form_data: FormResponse,
    user_id: &Uuid,
) -> Result<Option<Form>, BigError> {
    let current_stamp = Utc::now();

    let created_form = FormCreation {
        form_name: form_request.form_name,
        form_data: serde_json::to_value(form_data).context(SerdeSerializeSnafu)?,
        user_id: user_id.clone(),
        original_form_id: form_request.original_form_id,
        previous_form_id: form_request.previous_form_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::forms::dsl::forms;
    let new_form = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(forms)
            .values(&created_form)
            .get_result::<Form>(&mut conn)
            .optional();

        insert_response
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    Ok(new_form)
}
