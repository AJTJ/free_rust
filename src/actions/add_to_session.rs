use crate::auth_data::{SessionData, UniversalIdType};
use crate::Shared;
use actix_session::Session;
use actix_web::web;
use argon2::{self};
use async_graphql::Context;
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

pub async fn add_to_session(ctx: &Context<'_>, session_data: SessionData) {
    let shared_session = ctx.data_unchecked::<Shared<Session>>().clone();

    let id: UniversalIdType = Rng::gen::<UniversalIdType>(&mut rand::thread_rng());
    let encoded_session_id = general_purpose::STANDARD.encode(id);

    web::block(move || {
        shared_session
            .insert(encoded_session_id, session_data)
            .unwrap();
    })
    .await
    .expect("failure to store in session");
}
