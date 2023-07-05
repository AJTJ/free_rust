use actix_web::web;
use async_graphql::Context;
use redis::Commands;
use snafu::ResultExt;

use crate::{
    auth::utility::auth_data::RedisPool,
    utility::errors::{BigError, RedisSessionSnafu},
};

pub async fn remove_from_user_session(
    ctx: &Context<'_>,
    encoded_session_id: String,
) -> Result<bool, BigError> {
    let redis_pool = ctx.data::<RedisPool>().unwrap().clone();
    let result = web::block(move || {
        let mut redis_conn = redis_pool.get().unwrap();
        // TODO: not sure if the return value should be a bool
        let deleted = redis_conn.del::<String, bool>(encoded_session_id);
        deleted
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .context(RedisSessionSnafu);
    result
}
