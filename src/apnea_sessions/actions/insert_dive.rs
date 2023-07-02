use crate::{
    apnea_sessions::dto::dive_dto::{Dive, DiveCreation, DiveInput},
    auth::actions::get_user_id_from_token_and_session,
    graphql_schema::DbPool,
    utility::errors::BigError,
};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub async fn insert_dive(
    ctx: &Context<'_>,
    dive_session_id: Uuid,
    dive_data: DiveInput,
) -> Result<Dive, BigError> {
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;
    let new_dive = DiveCreation {
        discipline_type: dive_data.discipline_type,
        depth: dive_data.depth,
        distance: dive_data.distance,
        dive_time: dive_data.dive_time,
        dive_name: dive_data.dive_name,
        session_id: dive_session_id,
        user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::dives::dsl::dives;

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::insert_into(dives)
            .values(&new_dive)
            .get_result::<Dive>(&mut conn)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })
}
