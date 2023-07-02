// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::{Commands, RedisError};

use crate::auth::utility::auth_data::{RedisPool, SessionData};

pub async fn get_user_session_data(
    ctx: &Context<'_>,
    encoded_session_id: String,
) -> Result<SessionData, RedisError> {
    let redis_pool = ctx.data::<RedisPool>().unwrap().clone();

    let el = web::block(move || {
        let mut redis_conn = redis_pool.get().unwrap();
        redis_conn.get::<String, SessionData>(encoded_session_id)
    })
    .await
    .expect("error with web block in session_data retrieval");

    el
}
