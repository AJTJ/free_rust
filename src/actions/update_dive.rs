use crate::diesel::ExpressionMethods;
use crate::dto::dive_dto::{Dive, DiveUpdate};
use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::helpers::conversion_helpers::id_to_uuid;

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;

pub async fn update_dive(ctx: &Context<'_>, dive_mod_data: DiveUpdate) -> Result<Dive, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let my_dive_mod_data = dive_mod_data.clone();

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        use crate::schema::dives::dsl::{dives, id as dive_id, updated_at};
        diesel::update(dives)
            .filter(dive_id.eq(&my_dive_mod_data.id))
            .set((&my_dive_mod_data, updated_at.eq(Utc::now().naive_utc())))
            .get_result::<Dive>(&mut conn)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselUpdateError { source: e })
}
