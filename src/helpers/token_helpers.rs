use actix_web::cookie::time::{Duration as TimeDuration, OffsetDateTime};
use actix_web::cookie::Cookie;
use async_graphql::Context;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use tracing::info;

use crate::errors::BigError;
use crate::token_source::Token;

// COOKIE THINGS
pub const TOKEN_NAME: &str = "free_rust_token";
pub const CUSTOM_HEADER: &str = "Custom-Header";
pub const AUTHORIZATION_HEADER: &str = "Authorization";

#[derive(Serialize, Deserialize, Debug)]
pub struct CookieStruct {
    pub encoded_session_id: Option<String>,
}

// TODO: Should AUTHORIZATION tokens and COOKIES have expiries built-in?
pub fn create_cookie<'c>(encoded_session_id: String) -> Cookie<'c> {
    let cookie_struct = CookieStruct {
        encoded_session_id: Some(encoded_session_id),
    };

    let cookie = Cookie::build(TOKEN_NAME, json!(cookie_struct).to_string())
        .path("/")
        .secure(true)
        .max_age(TimeDuration::minutes(10080))
        .finish();

    cookie
}

pub fn create_expired_cookie<'c>() -> Cookie<'c> {
    let cookie_struct = CookieStruct {
        // TODO: Should this be like so?
        encoded_session_id: None,
    };

    let cookie = Cookie::build(TOKEN_NAME, json!(cookie_struct).to_string())
        .path("/")
        .secure(true)
        .expires(OffsetDateTime::UNIX_EPOCH)
        .finish();

    cookie
}

pub fn get_cookie_from_token(ctx: &Context<'_>) -> Result<CookieStruct, BigError> {
    let token = ctx.data::<Token>();

    match token {
        Ok(token) => {
            let c = Cookie::parse::<&str>(token.0.as_str())
                .map_err(|e| BigError::WrongCookieString { source: e })?;

            let (_, value) = c.name_value();
            serde_json::from_str(value).map_err(|e| BigError::SerdeParseError { source: e })
        }
        Err(e) => Err(BigError::AsyncIncorrectCookie { error: e }),
    }
}
