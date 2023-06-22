use crate::actions::{get_user_id_from_token_and_session, get_user_session_data};
use crate::diesel::ExpressionMethods;
use crate::dto::dive_session_dto::{DiveSession, DiveSessionCreation, DiveSessionInput};
use crate::dto::form_dto::{Form, FormCreation, FormInput};
use crate::dto::form_field_dto::{FormField, FormFieldCreation};
use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::helpers::conversion_helpers::{local_version_to_db_version, op_id_to_op_uuid};
use crate::helpers::form_helper::FormStructure;
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;

pub async fn add_form(
    ctx: &Context<'_>,
    form_input: FormInput,
) -> Result<(Form, Vec<FormField>), BigError> {
    let converted_input_form = FormStructure::from_input(form_input.form_structure_input)?;
    let validated_form = FormStructure::validate_form(&converted_input_form)?;
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;

    let my_original_form_id = op_id_to_op_uuid(&form_input.original_form_id)?;
    let my_previous_form_id = op_id_to_op_uuid(&form_input.previous_form_id)?;

    let created_form = FormCreation {
        form_name: form_input.form_name,
        template_version: local_version_to_db_version(&validated_form.form_template_version),
        user_id,
        original_form_id: my_original_form_id,
        previous_form_id: my_previous_form_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::forms::dsl::forms;
    let new_form_from_db = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(forms)
            .values(&created_form)
            .get_result::<Form>(&mut conn);

        insert_response
    })
    .await
    .map_err(|e| BigError::BlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    // Goint the DB route for now
    let new_form_fields: Vec<FormFieldCreation> = validated_form
        .all_fields
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let le = FormFieldCreation {
                item_order: Some(i.try_into().unwrap()),

                field_name: c.field_name.to_string(),
                field_value: c.field_value.clone(),
                category_name: c.category_name.to_string(),
                field_value_type: c.field_value_type.to_string(),

                form_id: new_form_from_db.id,
                user_id,

                created_at: current_stamp,
                updated_at: current_stamp,
                is_active: true,
            };
            le
        })
        .collect();

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::form_fields::dsl::form_fields;
    let all_inserted_form_fields = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(form_fields)
            .values(&new_form_fields)
            .get_results::<FormField>(&mut conn);

        insert_response
    })
    .await
    .map_err(|e| BigError::BlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    // TODO: Insert all these new form fields in the database

    Ok((new_form_from_db, all_inserted_form_fields))
}
