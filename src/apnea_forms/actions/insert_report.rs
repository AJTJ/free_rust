use crate::{
    apnea_forms::{
        dto::report_dto::{Report, ReportCreation, ReportDetailsInput},
        helpers::FormOutput,
    },
    auth::actions::get_user_id_from_token_and_session,
    graphql_schema::DbPool,
    utility::errors::{BigError, SerdeSerializeSnafu},
};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use serde_json::Value;
use snafu::ResultExt;

pub async fn insert_report(
    ctx: &Context<'_>,
    report_input: ReportDetailsInput,
    report_data: FormOutput,
) -> Result<Report, BigError> {
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;

    let created_report = ReportCreation {
        report_data: serde_json::to_value(report_data).context(SerdeSerializeSnafu)?,
        original_form_id: report_input.original_form_id,
        previous_report_id: report_input.previous_report_id,
        form_id: report_input.form_id,
        session_id: report_input.session_id,
        user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::reports::dsl::reports;
    let new_report = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(reports)
            .values(&created_report)
            .get_result::<Report>(&mut conn);

        insert_response
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    Ok(new_report)
}

// Another approach... or both?!?
// let new_report_fields: Vec<CompletedFormFieldCreation> = validated_report
//     .all_fields
//     .iter()
//     .enumerate()
//     .map(|(i, c)| {
//         let le = CompletedFormFieldCreation {
//             field_order: Some(i.try_into().unwrap()),

//             field_name: c.field_name.to_string(),
//             field_value: c.field_value.clone(),
//             category_name: c.category_name.to_string(),
//             field_value_type: c.field_value_type.iter().map(|t| t.to_string()).collect(),

//             report_id: new_created_form_from_db.id,
//             user_id,

//             created_at: current_stamp,
//             updated_at: current_stamp,
//             is_active: true,
//         };
//         le
//     })
//     .collect();

// let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

// use crate::schema::report_fields::dsl::report_fields;
// let all_inserted_report_fields = web::block(move || {
//     let mut conn = pool_ctx.get().unwrap();
//     let insert_response = diesel::insert_into(report_fields)
//         .values(&new_report_fields)
//         .get_results::<CompletedFormField>(&mut conn);

//     insert_response
// })
// .await
// .map_err(|e| BigError::ActixBlockingError { source: e })?
// .map_err(|e| BigError::DieselInsertError { source: e })?;

// Ok(CompletedFormOutput {
//     form: new_created_form_from_db,
//     fields: all_inserted_report_fields,
//     form_structure: FormStructureOutput::from(validated_report),
// })
