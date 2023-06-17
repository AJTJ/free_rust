use crate::auth_data::{SessionData, SharedRedisType};
// use actix_session::Session;
use actix_web::web;
use async_graphql::Context;
use redis::{Commands, RedisError};

pub async fn get_user_session_data(
    ctx: &Context<'_>,
    encoded_session_id: String,
) -> Result<SessionData, RedisError> {
    let session_arc = ctx.data::<SharedRedisType>().unwrap().clone();
    let el = web::block(move || {
        let redis_server = session_arc.lock().expect("error locking the redis mutex");

        let mut connection = redis_server
            .get_connection()
            .expect("error connecting to redis_server");

        connection.get::<String, SessionData>(encoded_session_id)
    })
    .await
    .expect("error with web block in session_data retrieval");

    el
}
