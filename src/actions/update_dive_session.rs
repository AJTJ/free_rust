use crate::dto::dive_session_dto::{DiveSessionModificationData, DiveSessionQueryData};
use crate::graphql_schema::DbPool;
use crate::{actions::get_dive_session_by_id, diesel::ExpressionMethods};

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{result::Error, RunQueryDsl};
use tracing::info;

pub async fn update_dive_session(
    ctx: &Context<'_>,
    session_mod_data: DiveSessionModificationData,
) -> DiveSessionQueryData {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let my_session_mod_data = session_mod_data.clone();
    let output_dive_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        use crate::schema::dive_sessions::dsl::{
            dive_sessions, unique_id as session_id, updated_at,
        };
        diesel::update(dive_sessions)
            .filter(session_id.eq(&my_session_mod_data.unique_id))
            .set((&my_session_mod_data, updated_at.eq(Utc::now().naive_utc())))
            .execute(&mut conn)
    })
    .await
    .expect("web::block error here?");

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let updated_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_dive_session_by_id(&mut conn, &session_mod_data.unique_id, None)
    })
    .await
    .expect("web::block error here?")
    .expect("error getting session");

    info!("the output: {:?}", output_dive_session);

    updated_session
}
