use crate::session_data::SessionData;
use crate::Shared;

use actix_session::Session;
use actix_web::web;
use async_graphql::Context;

pub async fn add_to_session(ctx: &Context<'_>, user_id: String, session_data: SessionData) {
    let shared_session = ctx.data_unchecked::<Shared<Session>>().clone();
    web::block(move || {
        shared_session.insert(user_id, session_data).unwrap();
    })
    .await
    .expect("failure to store in session");
}
