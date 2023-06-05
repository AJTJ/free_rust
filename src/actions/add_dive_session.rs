use crate::actions::get_user_session_data;
use crate::cookie_helpers::get_cookie_from_token;
use crate::diesel::ExpressionMethods;
use crate::dto::dive_session_dto::{
    DiveSessionCreationData, DiveSessionInputData, DiveSessionQueryData,
};
use crate::graphql_schema::DbPool;

use actix_web::web;
use async_graphql::{Context, Error};
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn add_dive_session(
    ctx: &Context<'_>,
    session_data: DiveSessionInputData,
) -> Result<DiveSessionQueryData, Error> {
    use crate::schema::dive_sessions::dsl::*;

    let current_stamp = Utc::now().naive_utc();
    let uuid = Uuid::new_v4();

    // TODO
    /*
     Ensure that a guard is in place that checks the user logged in status
     then take the user id out of the session data??
    */

    let cookie_data =
        get_cookie_from_token(ctx).expect("there should be cookie data, as this route is guarded");

    let user_session = get_user_session_data(ctx, cookie_data.encoded_session_id)
        .await
        .expect("expecting session to be there");

    let new_session = DiveSessionCreationData {
        session_id: uuid,
        start_time: session_data.start_time,
        end_time: session_data.end_time,
        session_name: session_data.session_name,
        user_id: user_session.user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let dive_session = web::block(move || {
        let conn = pool_ctx.get().unwrap();
        diesel::insert_into(dive_sessions)
            .values(&new_session)
            .execute(&conn)
            .expect("diesel insert new user error");

        dive_sessions
            .filter(session_id.eq(&uuid))
            .first::<DiveSessionQueryData>(&conn)
            .expect("error loading person that was just inserted")
    })
    .await
    .expect("web::block error here?");

    Ok(dive_session)
}
