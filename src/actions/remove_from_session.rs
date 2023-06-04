use crate::{auth_data::SharedRedisType, Shared};

// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::Commands;
use tracing::info;

pub async fn remove_from_session(ctx: &Context<'_>, decoded_universal_id: String) {
    let session_arc = ctx.data::<SharedRedisType>().unwrap().clone();

    web::block(move || {
        let redis_server = session_arc.lock().unwrap();
        let mut connection = redis_server.get_connection().unwrap();
        let update_session_data: String = connection
            .del(decoded_universal_id)
            .expect("expecting redis logout to produce a String");

        info!("the removed sesh: {:?}", update_session_data);
    })
    .await
    .expect("failure to store in session");
}
