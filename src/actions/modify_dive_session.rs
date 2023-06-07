use crate::actions::{get_dive_session_by_id, get_dive_sessions_by_user, get_user_session_data};
use crate::cookie_helpers::get_cookie_from_token;
use crate::diesel::ExpressionMethods;
use crate::dto::db_query_dto::DBQueryObject;
use crate::dto::dive_session_dto::{
    DiveSessionCreationData, DiveSessionInputData, DiveSessionModificationData,
    DiveSessionQueryData, DiveSessionQueryInput,
};
use crate::graphql_schema::DbPool;

use actix_web::web;
use async_graphql::{Context, Error};
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn modify_dive_session(
    ctx: &Context<'_>,
    session_mod_data: DiveSessionModificationData,
    db_query_ob: DBQueryObject,
) -> Result<DiveSessionQueryData, Error> {
    use crate::schema::dive_sessions::dsl::*;
    let current_stamp = Utc::now().naive_utc();

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let dive_session = web::block(move || {
        let conn = pool_ctx.get().unwrap();
        diesel::update(dive_sessions.filter(session_id.eq(&session_mod_data.session_id))).set((
            start_time.eq(session_mod_data.start_time.unwrap_or(start_time)),
            end_time.eq(session_mod_data.end_time.unwrap_or(end_time)),
        ))

        // get_dive_session_by_id(&mut conn, session_mod_data.session_id, db_query_ob)
    })
    .await
    .expect("web::block error here?");

    // let dive_session = web::block(move || {
    //     let conn = pool_ctx.get().unwrap();
    //     diesel::insert_into(dive_sessions)
    //         .values(&new_session)
    //         .execute(&conn)
    //         .expect("diesel insert new user error");

    //     dive_sessions
    //         .filter(session_id.eq(&uuid))
    //         .first::<DiveSessionQueryData>(&conn)
    //         .expect("error loading person that was just inserted")
    // })
    // .await
    // .expect("web::block error here?");

    Ok(dive_session)
}
