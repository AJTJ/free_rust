use crate::apnea_forms::forms_interface::ReportResponse;
use crate::apnea_sessions::dto::apnea_session_dto::{ApneaSession, ApneaSessionCreation};
use crate::graphql_schema::DbPool;
use crate::utility::errors::{BigError, SerdeSerializeSnafu};
use crate::{apnea_sessions::dto::apnea_session_dto::ApneaSessionInput, diesel::ExpressionMethods};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use snafu::ResultExt;
use tracing::info;
use uuid::Uuid;

pub async fn insert_apnea_session(
    ctx: &Context<'_>,
    session_input: ApneaSessionInput,
    user_id: &Uuid,
) -> Result<ApneaSession, BigError> {
    use crate::schema::apnea_sessions::dsl::apnea_sessions;

    let uuid = Uuid::new_v4();
    let current_stamp = Utc::now();

    let new_session = ApneaSessionCreation {
        report_data: serde_json::to_value(ReportResponse::from_input(session_input.report_data))
            .context(SerdeSerializeSnafu)?,

        form_id: session_input.form_id,
        original_form_id: session_input.original_form_id,
        previous_session_id: session_input.previous_session_id,
        user_id: user_id.clone(),

        id: uuid,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let new_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let response = diesel::insert_into(apnea_sessions)
            .values(&new_session)
            .get_result::<ApneaSession>(&mut conn);
        response
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    // if let (Some(report_input), Some(report_details)) =
    //     (session_input.session_report, report_details)
    // {
    //     insert_report(
    //         ctx,
    //         &new_session.id,
    //         report_details,
    //         FormResponse::from_input(report_input),
    //         user_id,
    //     )
    //     .await?;
    // };

    // let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    // let refetched_session = web::block(move || {
    //     let mut conn = pool_ctx.get().unwrap();
    //     get_apnea_session(&mut conn, &new_session.id)
    // })
    // .await
    // .map_err(|e| BigError::ActixBlockingError { source: e })?
    // .map_err(|e| BigError::DieselInsertError { source: e })?;

    info!("new sesssion: {new_session:?}");

    Ok(new_session)
}
