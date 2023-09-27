// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::Commands;
use snafu::ResultExt;

use crate::{
    auth::utility::auth_data::{RedisPool, SessionData},
    utility::errors::{BigError, RedisSessionSnafu},
};

pub async fn get_user_session(
    ctx: &Context<'_>,
    encoded_session_id: String,
) -> Result<SessionData, BigError> {
    let redis_pool = ctx.data::<RedisPool>().unwrap().clone();

    web::block(move || {
        let mut redis_conn = redis_pool.get().unwrap();
        redis_conn.get::<String, SessionData>(encoded_session_id)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .context(RedisSessionSnafu)
}

// .map_err(|e| BigError::ActixBlockingError { source: e })?
//         .context(DieselQuerySnafu)?;
