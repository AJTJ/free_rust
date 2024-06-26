use crate::apnea_forms::form_v1::unique_apneas::UniqueApneaActivity;
use crate::apnea_forms::forms_interface::{ReportRequest, StoredReport};
use crate::apnea_sessions::dto::apnea_session_dto::ApneaSessionInput;
use crate::apnea_sessions::dto::apnea_session_dto::{ApneaSession, ApneaSessionCreation};
use crate::apnea_sessions::dto::unique_apnea_dto::{UniqueApnea, UniqueApneaCreation};
use crate::graphql_schema::DbPool;
use crate::utility::errors::{BigError, SerdeSerializeSnafu};
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

    let current_stamp = Utc::now();

    let new_session = ApneaSessionCreation {
        report_data: serde_json::to_value(StoredReport::from_input(
            session_input.report_data.clone(),
        ))
        .context(SerdeSerializeSnafu)?,

        form_id: session_input.form_id,
        original_form_id: session_input.original_form_id,
        previous_session_id: session_input.previous_session_id,
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

    match session_input.report_data {
        ReportRequest::V1(report) => {
            let mut all_input_unique_apneas: Vec<UniqueApneaCreation> = vec![];
            if let Some(dives) = report.deep_dives {
                for dive in dives.iter() {
                    all_input_unique_apneas.push(UniqueApneaCreation {
                        activity_data: serde_json::to_value(UniqueApneaActivity::DeepDiveV1(
                            dive.clone(),
                        ))
                        .context(SerdeSerializeSnafu)?,
                        session_id: new_session.id,
                        user_id: user_id.clone(),

                        created_at: current_stamp,
                        updated_at: current_stamp,
                        is_active: true,
                    })
                }
            }

            if let Some(static_holds) = report.static_holds {
                for sta in static_holds.iter() {
                    all_input_unique_apneas.push(UniqueApneaCreation {
                        activity_data: serde_json::to_value(UniqueApneaActivity::StaticHoldsV1(
                            sta.clone(),
                        ))
                        .context(SerdeSerializeSnafu)?,
                        session_id: new_session.id,
                        user_id: user_id.clone(),

                        created_at: current_stamp,
                        updated_at: current_stamp,
                        is_active: true,
                    })
                }
            }

            if let Some(dynamic_dives) = report.dynamic_dives {
                for single_dyn in dynamic_dives.iter() {
                    all_input_unique_apneas.push(UniqueApneaCreation {
                        activity_data: serde_json::to_value(UniqueApneaActivity::DynDiveV1(
                            single_dyn.clone(),
                        ))
                        .context(SerdeSerializeSnafu)?,
                        session_id: new_session.id,
                        user_id: user_id.clone(),

                        created_at: current_stamp,
                        updated_at: current_stamp,
                        is_active: true,
                    })
                }
            }

            use crate::schema::unique_apneas::dsl::unique_apneas;

            let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

            let _all_dive_inputs = web::block(move || {
                let mut conn = pool_ctx.get().unwrap();
                let response = diesel::insert_into(unique_apneas)
                    .values(&all_input_unique_apneas)
                    .get_results::<UniqueApnea>(&mut conn);
                response
            })
            .await
            .map_err(|e| BigError::ActixBlockingError { source: e })?
            .map_err(|e| BigError::DieselInsertError { source: e })?;
        }
    }

    info!("new sesssion: {new_session:?}");

    Ok(new_session)
}

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

// if let Some(input_activities) = session_input.unique_apnea_activities {
//     use crate::schema::unique_apneas::dsl::unique_apneas;
//     let mut all_input_unique_apneas: Vec<UniqueApneaCreation> = vec![];

//     for act in input_activities.iter() {
//         all_input_unique_apneas.push(UniqueApneaCreation {
//             activity_data: serde_json::to_value(UniqueApneaActivity::from_input(act.clone()))
//                 .context(SerdeSerializeSnafu)?,
//             session_id: new_session.id,
//             user_id: user_id.clone(),

//             created_at: current_stamp,
//             updated_at: current_stamp,
//             is_active: true,
//         })
//     }

//     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

//     let all_dive_inputs = web::block(move || {
//         let mut conn = pool_ctx.get().unwrap();
//         let response = diesel::insert_into(unique_apneas)
//             .values(&all_input_unique_apneas)
//             .get_results::<UniqueApnea>(&mut conn);
//         response
//     })
//     .await
//     .map_err(|e| BigError::ActixBlockingError { source: e })?
//     .map_err(|e| BigError::DieselInsertError { source: e })?;
// };
