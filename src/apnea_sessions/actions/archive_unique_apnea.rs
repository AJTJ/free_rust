use crate::{
    apnea_sessions::dto::unique_apnea_dto::UniqueApnea, diesel::ExpressionMethods,
    graphql_schema::DbPool, utility::errors::BigError,
};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{OptionalExtension, /* QueryDsl, */ RunQueryDsl};
use uuid::Uuid;

pub async fn archive_unique_apnea(
    ctx: &Context<'_>,
    unique_apnea_input_id: &Uuid,
    input_user_id: &Uuid,
) -> Result<Option<UniqueApnea>, BigError> {
    use crate::schema::unique_apneas::dsl::{
        archived_at, archived_by, id, is_active, unique_apneas,
    };

    let current_stamp = Utc::now();
    let my_unique_apnea_id = unique_apnea_input_id.clone();
    let my_user_id = input_user_id.clone();
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let archived_unique_apnea = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::update(unique_apneas)
            .filter(id.eq(&my_unique_apnea_id))
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

    Ok(archived_unique_apnea)
}
