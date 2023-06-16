use crate::{cookie_helpers::get_cookie_from_token, errors::BigError};
// use actix_session::Session;
use super::get_user_session_data;
use async_graphql::Context;
use uuid::Uuid;

pub async fn get_user_id_from_cookie_session(ctx: &Context<'_>) -> Result<Uuid, BigError> {
    let cookie = get_cookie_from_token(ctx)?;

    let user_session = get_user_session_data(ctx, cookie.encoded_session_id)
        .await
        .map_err(|e| BigError::RedisSessionError { source: e })?;
    Ok(user_session.user_id)
}
