use crate::apnea_forms::dto::form_dto::{Form, FormCreation, FormDetailsInput};
use crate::apnea_forms::helpers::FormOutput;
use crate::auth::actions::get_user_id_from_auth;
use crate::graphql_schema::DbPool;
use crate::utility::errors::{BigError, SerdeSerializeSnafu};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use serde_json;
use snafu::ResultExt;

pub async fn insert_form(
    ctx: &Context<'_>,
    form_input: FormDetailsInput,
    form_data: FormOutput,
) -> Result<Form, BigError> {
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_auth(ctx).await?;

    let created_form = FormCreation {
        form_name: form_input.form_name,
        form_data: serde_json::to_value(form_data).context(SerdeSerializeSnafu)?,
        user_id,
        original_form_id: form_input.original_form_id,
        previous_form_id: form_input.previous_form_id,
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
            .get_result::<Form>(&mut conn);

        insert_response
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    Ok(new_form)
}

// Goint the DB route for now
// let new_form_fields: Vec<FormFieldCreation> = validated_form
//     .all_fields
//     .iter()
//     .enumerate()
//     .map(|(i, c)| {
//         let le = FormFieldCreation {
//             field_order: Some(i.try_into().unwrap()),

//             field_name: c.field_name.to_string(),
//             field_value: c.field_value.clone(),
//             category_name: c.category_name.to_string(),
//             field_value_type: c.field_value_type.iter().map(|t| t.to_string()).collect(),

//             form_id: new_form_from_db.id,
//             user_id,

//             created_at: current_stamp,
//             updated_at: current_stamp,
//             is_active: true,
//         };
//         le
//     })
//     .collect();

// let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

// use crate::schema::form_fields::dsl::form_fields;
// let all_inserted_form_fields = web::block(move || {
//     let mut conn = pool_ctx.get().unwrap();
//     let insert_response = diesel::insert_into(form_fields)
//         .values(&new_form_fields)
//         .get_results::<FormField>(&mut conn);

//     insert_response
// })
// .await
// .map_err(|e| BigError::ActixBlockingError { source: e })?
// .map_err(|e| BigError::DieselInsertError { source: e })?;
