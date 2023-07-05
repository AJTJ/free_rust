// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::Commands;
use snafu::ResultExt;

use crate::{
    auth::utility::auth_data::{RedisPool, SessionData},
    utility::errors::{ActixBlockingSnafu, BigError, RedisSessionSnafu},
};

pub async fn insert_into_user_session(
    ctx: &Context<'_>,
    session_data: SessionData,
    encoded_session_id: String,
) -> Result<bool, BigError> {
    let redis_pool = ctx.data::<RedisPool>().unwrap().clone();

    web::block(move || {
        let mut redis_conn = redis_pool.get().unwrap();
        // TODO: Still not convinced this should be a bool
        redis_conn.set::<String, SessionData, bool>(encoded_session_id, session_data)
    })
    .await
    .context(ActixBlockingSnafu)?
    .context(RedisSessionSnafu)
}
