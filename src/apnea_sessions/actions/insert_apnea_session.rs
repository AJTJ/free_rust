use crate::apnea_forms::actions::insert_report::insert_report;
use crate::apnea_forms::dto::report_dto::ReportDetails;
use crate::apnea_forms::helpers::FormResponse;
use crate::apnea_sessions::actions::get_apnea_session;
use crate::apnea_sessions::dto::apnea_session_dto::{ApneaSession, ApneaSessionCreation};
use crate::auth::actions::get_user_id_from_auth;
use crate::graphql_schema::DbPool;
use crate::utility::errors::BigError;
use crate::{apnea_sessions::dto::apnea_session_dto::ApneaSessionInput, diesel::ExpressionMethods};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub async fn insert_apnea_session(
    ctx: &Context<'_>,
    session_input: ApneaSessionInput,
    report_details: Option<ReportDetails>,
    user_id: &Uuid,
) -> Result<ApneaSession, BigError> {
    use crate::schema::apnea_sessions::dsl::apnea_sessions;

    let uuid = Uuid::new_v4();
    let current_stamp = Utc::now();

    let new_session = ApneaSessionCreation {
        id: uuid,
        start_time: session_input.start_time,
        end_time: session_input.end_time,
        session_name: session_input.session_name,
        user_id: user_id.clone(),
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

    if let (Some(report_input), Some(report_details)) =
        (session_input.session_report, report_details)
    {
        insert_report(
            ctx,
            &new_session.id,
            report_details,
            FormResponse::from_input(report_input),
            user_id,
        )
        .await?;
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let refetched_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_apnea_session(&mut conn, &new_session.id)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    Ok(refetched_session)
}
