use crate::Shared;

use actix_session::Session;
use actix_web::web;
use async_graphql::Context;

pub async fn remove_from_session(ctx: &Context<'_>, user_id: String) {
    let shared_session = ctx.data_unchecked::<Shared<Session>>().clone();
    web::block(move || {
        shared_session.remove(&user_id);
    })
    .await
    .expect("web::block - failure to remove from session");
}
