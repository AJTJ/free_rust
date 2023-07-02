use actix_web::web;
use async_graphql::Context;
use redis::Commands;

use crate::auth::utility::auth_data::RedisPool;

pub async fn remove_from_user_session(ctx: &Context<'_>, encoded_session_id: String) {
    let redis_pool = ctx.data::<RedisPool>().unwrap().clone();
    web::block(move || {
        let mut redis_conn = redis_pool.get().unwrap();
        // TODO: not sure if the return value should be a bool
        let deleted = redis_conn.del::<String, bool>(encoded_session_id);
        deleted.expect("expecting redis logout to produce a String")
    })
    .await
    .expect("failure to remove from session");
}
