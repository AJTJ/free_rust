use crate::actions::{get_user_id_from_token_and_session, get_user_session_data};
use crate::diesel::ExpressionMethods;
use crate::dto::dive_dto::{DiveCreation, DiveInput, DiveQuery};
use crate::dto::dive_session_dto::{DiveSessionCreation, DiveSessionInput, DiveSessionQuery};
use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::helpers::token_helpers::get_cookie_from_token;

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn add_dive(
    ctx: &Context<'_>,
    dive_session_id: Uuid,
    dive_data: DiveInput,
) -> Result<DiveQuery, BigError> {
    let current_stamp = Utc::now().naive_utc();
    let uuid = Uuid::new_v4();

    let user_id = get_user_id_from_token_and_session(ctx).await?;

    let new_dive = DiveCreation {
        id: uuid,
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

    use crate::schema::dives::dsl::{dives, id as dive_id};

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::insert_into(dives)
            .values(&new_dive)
            .get_result::<DiveQuery>(&mut conn)
    })
    .await
    .map_err(|e| BigError::BlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })
}
