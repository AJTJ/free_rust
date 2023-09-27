use crate::{
    apnea_sessions::dto::apnea_session_dto::ApneaSession, diesel::ExpressionMethods,
    graphql_schema::DbPool, utility::errors::BigError,
};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{OptionalExtension, /* PgConnection, QueryDsl, */ RunQueryDsl};
use uuid::Uuid;

pub async fn archive_session(
    ctx: &Context<'_>,
    session_id_input: &Uuid,
    input_user_id: &Uuid,
) -> Result<Option<ApneaSession>, BigError> {
    use crate::schema::apnea_sessions::dsl::{
        apnea_sessions, archived_at, archived_by, id, is_active,
    };

    let current_stamp = Utc::now();
    let my_session = session_id_input.clone();
    let my_user_id = input_user_id.clone();
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let archived_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::update(apnea_sessions)
            .filter(id.eq(&my_session))
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

    Ok(archived_session)
}
