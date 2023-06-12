use crate::actions::get_dive_by_id;
use crate::dto::dive_dto::{DiveModificationData, DiveQueryData};
use crate::dto::dive_session_dto::{DiveSessionModificationData, DiveSessionQueryData};
use crate::graphql_schema::DbPool;
use crate::{actions::get_dive_session_by_id, diesel::ExpressionMethods};

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{result::Error, RunQueryDsl};
use tracing::info;

pub async fn update_dive(ctx: &Context<'_>, dive_mod_data: DiveModificationData) -> DiveQueryData {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let my_dive_mod_data = dive_mod_data.clone();
    let output_dive = web::block(move || {
        let conn = pool_ctx.get().unwrap();
        use crate::schema::dives::dsl::{dives, unique_id as dive_id, updated_at};
        diesel::update(dives)
            .filter(dive_id.eq(&my_dive_mod_data.unique_id))
            .set((&my_dive_mod_data, updated_at.eq(Utc::now().naive_utc())))
            .execute(&conn)
    })
    .await
    .expect("web::block error here?");

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let updated_dive = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_dive_by_id(&mut conn, dive_mod_data.unique_id)
    })
    .await
    .expect("web::block error here?")
    .expect("error getting session");

    info!("the output: {:?}", output_dive);

    updated_dive
}
