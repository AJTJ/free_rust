use crate::auth_data::{SessionData, SharedRedisType};
// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::Commands;

// TODO: need to create a universal key to encrypt session data

pub async fn add_to_user_session(
    ctx: &Context<'_>,
    session_data: SessionData,
    encoded_session_id: String,
) {
    let session_arc = ctx.data::<SharedRedisType>().unwrap().clone();

    web::block(move || {
        let redis_server = session_arc.lock().expect("error locking the redis mutex");

        let mut connection = redis_server
            .get_connection()
            .expect("error connecting to redis_server");

        connection
            .set::<String, SessionData, bool>(encoded_session_id, session_data)
            .expect("should have updated teh session data");
    })
    .await
    .expect("failure to store in session");
}
