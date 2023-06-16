use crate::actions::get_user_session_data;
use crate::diesel::ExpressionMethods;
use crate::dto::dive_dto::{DiveCreationData, DiveInputData, DiveQueryData};
use crate::dto::dive_session_dto::{
    DiveSessionCreationData, DiveSessionInputData, DiveSessionQueryData,
};
use crate::graphql_schema::DbPool;
use crate::helpers::cookie_helpers::get_cookie_from_token;

use actix_web::web;
use async_graphql::{Context, Error};
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn add_dive(
    ctx: &Context<'_>,
    dive_session_id: Uuid,
    dive_data: DiveInputData,
) -> Result<DiveQueryData, Error> {
    let current_stamp = Utc::now().naive_utc();
    let uuid = Uuid::new_v4();

    let cookie_data =
        get_cookie_from_token(ctx).expect("there should be cookie data, as this route is guarded");

    let user_session = get_user_session_data(ctx, cookie_data.encoded_session_id)
        .await
        .expect("expecting session to be there");

    let new_dive = DiveCreationData {
        id: uuid,
        discipline_type: dive_data.discipline_type,
        depth: dive_data.depth,
        distance: dive_data.distance,
        dive_time: dive_data.dive_time,
        dive_name: dive_data.dive_name,
        session_id: dive_session_id,
        user_id: user_session.user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::dives::dsl::{dives, id as dive_id};

    let dive_session = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::insert_into(dives)
            .values(&new_dive)
            .execute(&mut conn)
            .expect("diesel insert new dive error");

        dives
            .filter(dive_id.eq(&uuid))
            .first::<DiveQueryData>(&mut conn)
            .expect("error loading dive that was just inserted")
    })
    .await
    .expect("web::block error here?");

    Ok(dive_session)
}
