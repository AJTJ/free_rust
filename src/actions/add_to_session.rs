use crate::auth_data::{SessionData, SessionKeyValue, SharedRedisType};
// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::{Commands, Connection};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

// TODO: need to create a universal key to encrypt session data

pub async fn add_to_session(
    ctx: &Context<'_>,
    session_data: SessionData,
    encoded_session_id: String,
) {
    info!("pre sesh");
    let session_arc = ctx.data::<SharedRedisType>().unwrap().clone();

    web::block(move || {
        let redis_server = session_arc.lock().expect("error locking the redis mutex");
        info!("MEOW1");
        let mut connection = redis_server
            .get_connection()
            .expect("error connecting to redis_server");
        info!("MEOW2");
        let update_session_data =
            connection.set::<&String, SessionData, SessionData>(&encoded_session_id, session_data);
        info!("MEOW3");

        info!("the updated sesh: {:?}", update_session_data);
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
