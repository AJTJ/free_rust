use crate::cookie_helpers::{get_expired_cookie, CookieStruct};
use crate::token_source::Token;
use actix_web::cookie::Cookie;
use actix_web::http::header::SET_COOKIE;
use async_graphql::Context;
use tracing::info;

use super::remove_from_session;

pub async fn logout(ctx: &Context<'_>) {
    let token = ctx.data::<Token>();

    match token {
        Ok(token) => {
            let c = Cookie::parse::<&str>(token.0.as_str()).unwrap();
            let (_, value) = c.name_value();
            let cookie_data: CookieStruct =
                serde_json::from_str(value).expect("parsing cookie error");

            remove_from_session(ctx, cookie_data.encoded_session_id).await;
        }
        Err(e) => info!("No token/cookie: {:?}", e),
    }

    let expired_cookied = get_expired_cookie();
    ctx.insert_http_header(SET_COOKIE, expired_cookied.to_string());
}
