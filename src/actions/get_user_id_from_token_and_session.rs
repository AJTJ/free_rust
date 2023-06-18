use crate::{errors::BigError, helpers::token_helpers::get_cookie_from_token};
// use actix_session::Session;
use super::get_user_session_data;
use async_graphql::Context;
use uuid::Uuid;

pub async fn get_user_id_from_token_and_session(ctx: &Context<'_>) -> Result<Uuid, BigError> {
    let cookie = get_cookie_from_token(ctx)?;

    if let Some(session_id) = cookie.encoded_session_id {
        let user_session = get_user_session_data(ctx, session_id)
            .await
            .map_err(|e| BigError::RedisSessionError { source: e })?;
        Ok(user_session.user_id)
    } else {
        Err(BigError::NoSessionIDOnToken)
    }
}
