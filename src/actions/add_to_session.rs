use crate::auth_data::{SessionData, SessionKeyValue, SharedRedisType};
// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::{Commands, Connection};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub async fn add_to_session(
    ctx: &Context<'_>,
    session_data: SessionData,
    encoded_session_id: String,
) {
    info!("pre sesh");
    let session_arc = ctx.data::<SharedRedisType>().unwrap().clone();

    web::block(move || {
        let redis_server = session_arc.lock().unwrap();
        let mut connection = redis_server.get_connection().unwrap();
        let update_session_data =
            connection.set::<&String, SessionData, SessionData>(&encoded_session_id, session_data);
    })
    .await
    .expect("failure to store in session");

    info!("post sesh");
}

/*
std::thread::sleep(Duration::from_secs(5));
    shared_session
        .insert(encoded_session_id, session_data)
        .unwrap();
info!("after session add");

        info!(
            "got it: {:?}",
            shared_session.get::<SessionData>(&encoded_session_id)
        )
 */
