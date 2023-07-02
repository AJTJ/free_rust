// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::Commands;

use crate::auth::utility::auth_data::{RedisPool, SessionData};

pub async fn insert_into_user_session(
    ctx: &Context<'_>,
    session_data: SessionData,
    encoded_session_id: String,
) {
    let redis_pool = ctx.data::<RedisPool>().unwrap().clone();

    web::block(move || {
        let mut redis_conn = redis_pool.get().unwrap();

        redis_conn
            .set::<String, SessionData, bool>(encoded_session_id, session_data)
            .expect("should have updated teh session data");
    })
    .await
    .expect("failure to store in session");
}
