use crate::cookie_helpers::{create_expired_cookie, get_cookie_from_token, CookieStruct};
use crate::token_source::Token;
use actix_web::cookie::Cookie;
use actix_web::http::header::SET_COOKIE;
use async_graphql::Context;
use tracing::info;

use super::remove_from_session;

pub async fn logout(ctx: &Context<'_>) {
    if let Some(cookie_data) = get_cookie_from_token(ctx) {
        remove_from_session(ctx, cookie_data.encoded_session_id).await;
    }

    let expired_cookied = create_expired_cookie();
    ctx.insert_http_header(SET_COOKIE, expired_cookied.to_string());
}
