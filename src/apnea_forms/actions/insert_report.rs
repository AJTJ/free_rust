// use crate::{
//     apnea_forms::{
//         dto::report_dto::{Report, ReportCreation, ReportDetails},
//         helpers::FormResponse,
//     },
//     auth::actions::get_user_id_from_auth,
//     graphql_schema::DbPool,
//     utility::errors::{BigError, SerdeSerializeSnafu},
// };
// use actix_web::web;
// use async_graphql::Context;
// use chrono::Utc;
// use diesel::{
//     BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
//     RunQueryDsl,
// };
// use snafu::ResultExt;
// use uuid::Uuid;

// pub async fn insert_report(
//     ctx: &Context<'_>,
//     session_id: &Uuid,
//     report_input: ReportDetails,
//     report_data: FormResponse,
//     user_id: &Uuid,
// ) -> Result<Option<Report>, BigError> {
//     let current_stamp = Utc::now();

//     let created_report = ReportCreation {
//         report_data: serde_json::to_value(report_data).context(SerdeSerializeSnafu)?,
//         original_form_id: report_input.original_form_id,
//         previous_report_id: report_input.previous_report_id,
//         form_id: report_input.form_id,
//         session_id: *session_id,
//         user_id: user_id.clone(),
//         created_at: current_stamp,
//         updated_at: current_stamp,
//         is_active: true,
//     };

//     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

//     use crate::schema::reports::dsl::reports;
//     let new_report = web::block(move || {
//         let mut conn = pool_ctx.get().unwrap();
//         let insert_response = diesel::insert_into(reports)
//             .values(&created_report)
//             .get_result::<Report>(&mut conn)
//             .optional();

//         insert_response
//     })
//     .await
//     .map_err(|e| BigError::ActixBlockingError { source: e })?
//     .map_err(|e| BigError::DieselInsertError { source: e })?;

//     Ok(new_report)
// }
