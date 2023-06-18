use crate::{
    errors::BigError,
    helpers::token_helpers::{create_expired_cookie, get_cookie_from_token},
};
use actix_web::http::header::{AUTHORIZATION, SET_COOKIE};
use async_graphql::Context;
use tracing::info;

use super::remove_from_user_session;

pub async fn logout(ctx: &Context<'_>) -> Result<(), BigError> {
    if let Ok(cookie_data) = get_cookie_from_token(ctx) {
        if let Some(s) = cookie_data.encoded_session_id {
            remove_from_user_session(ctx, s).await
        } else {
            return Err(BigError::NoSessionIDOnToken);
        }
    }
    let expired_cookied = create_expired_cookie();
    ctx.insert_http_header(SET_COOKIE, expired_cookied.to_string());
    ctx.insert_http_header(AUTHORIZATION, expired_cookied.to_string());
    Ok(())
}
