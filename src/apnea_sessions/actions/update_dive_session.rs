use crate::apnea_sessions::dto::dive_session_dto::DiveSession;
use crate::graphql_schema::DbPool;
use crate::utility::errors::BigError;
use crate::{apnea_sessions::dto::dive_session_dto::DiveSessionUpdate, diesel::ExpressionMethods};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;

use super::get_dive_session;

// TODO: better error handling
pub async fn update_dive_session(
    ctx: &Context<'_>,
    session_mod_data: DiveSessionUpdate,
) -> Result<DiveSession, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let my_session_mod_data = session_mod_data.clone();

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        use crate::schema::dive_sessions::dsl::{dive_sessions, id as session_id, updated_at};
        let update_statement = diesel::update(dive_sessions)
            .filter(session_id.eq(&my_session_mod_data.id))
            .set((&my_session_mod_data, updated_at.eq(Utc::now().naive_utc())))
            .execute(&mut conn);

        update_statement
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselUpdateError { source: e })?;

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let my_session_mod_data = session_mod_data.clone();

    let updated_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_dive_session(&mut conn, &my_session_mod_data.id, None)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselQueryError { source: e });

    updated_session
}
