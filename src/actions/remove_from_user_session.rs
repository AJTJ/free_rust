use crate::auth_data::SharedRedisType;
use actix_web::web;
use async_graphql::Context;
use redis::Commands;

pub async fn remove_from_user_session(ctx: &Context<'_>, encoded_session_id: String) {
    let session_arc = ctx.data::<SharedRedisType>().unwrap().clone();

    web::block(move || {
        let redis_server = session_arc.lock().unwrap();
        let mut connection = redis_server.get_connection().unwrap();
        // TODO: not sure if the return value should be a bool
        let deleted = connection.del::<String, bool>(encoded_session_id);
        deleted.expect("expecting redis logout to produce a String")
    })
    .await
    .expect("failure to remove from session");
}
