use crate::dto::dive_session_dto::{DiveSession, DiveSessionUpdate};

use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::{actions::get_dive_session_by_id, diesel::ExpressionMethods};

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::QueryResult;
use diesel::{result::Error, RunQueryDsl};
use tracing::info;

// TODO: better error handling
pub async fn update_dive_session(
    ctx: &Context<'_>,
    session_mod_data: DiveSessionUpdate,
) -> Result<DiveSession, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let my_session_mod_data = session_mod_data.clone();
    let output_dive_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        use crate::schema::dive_sessions::dsl::{dive_sessions, id as session_id, updated_at};
        let update_statement = diesel::update(dive_sessions)
            .filter(session_id.eq(&my_session_mod_data.id))
            .set((&my_session_mod_data, updated_at.eq(Utc::now().naive_utc())))
            .execute(&mut conn);

        update_statement
    })
    .await
    .expect("web::block error here?")
    .map_err(|e| BigError::DieselUpdateError { source: e });

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let updated_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_dive_session_by_id(&mut conn, &session_mod_data.id, None)
    })
    .await
    .expect("web::block error here?")
    .map_err(|e| BigError::DieselQueryError { source: e });

    updated_session
}
