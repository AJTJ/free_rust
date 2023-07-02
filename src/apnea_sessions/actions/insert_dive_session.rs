use crate::apnea_sessions::dto::dive_session_dto::{DiveSession, DiveSessionCreation};
use crate::auth::actions::get_user_id_from_auth;
use crate::graphql_schema::DbPool;
use crate::utility::errors::BigError;
use crate::{apnea_sessions::dto::dive_session_dto::DiveSessionInput, diesel::ExpressionMethods};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub async fn add_dive_session(
    ctx: &Context<'_>,
    session_data: DiveSessionInput,
) -> Result<DiveSession, BigError> {
    use crate::schema::dive_sessions::dsl::dive_sessions;

    let current_stamp = Utc::now().naive_utc();
    let uuid = Uuid::new_v4();

    let user_id = get_user_id_from_auth(ctx).await?;

    let new_session = DiveSessionCreation {
        id: uuid,
        start_time: session_data.start_time,
        end_time: session_data.end_time,
        session_name: session_data.session_name,
        user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let response = diesel::insert_into(dive_sessions)
            .values(&new_session)
            .get_result::<DiveSession>(&mut conn);
        response
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })
}
