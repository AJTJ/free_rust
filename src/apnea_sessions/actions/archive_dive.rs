use crate::{
    apnea_forms::dto::form_dto::Form, apnea_sessions::dto::dive_dto::Dive,
    diesel::ExpressionMethods, graphql_schema::DbPool, utility::errors::BigError,
};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn archive_dive(
    ctx: &Context<'_>,
    dive_id_input: &Uuid,
    input_user_id: &Uuid,
) -> Result<Option<Dive>, BigError> {
    use crate::schema::dives::dsl::{archived_at, archived_by, dives, id, is_active};

    let current_stamp = Utc::now();
    let my_dive_id = dive_id_input.clone();
    let my_user_id = input_user_id.clone();
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let archived_dive = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::update(dives)
            .filter(id.eq(&my_dive_id))
            .set((
                is_active.eq(false),
                archived_at.eq(current_stamp),
                archived_by.eq(my_user_id),
            ))
            .get_result(&mut conn)
            .optional()
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    Ok(archived_dive)
}
