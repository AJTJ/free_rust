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

use crate::token_source::Token;

// COOKIE THINGS
pub const COOKIE_NAME: &str = "free_rust_cookie";

#[derive(Serialize, Deserialize)]
pub struct CookieStruct {
    pub encoded_session_id: String,
}

pub fn create_cookie<'c>(encoded_session_id: String) -> Cookie<'c> {
    let cookie_struct = CookieStruct { encoded_session_id };

    let cookie = Cookie::build(COOKIE_NAME, json!(cookie_struct).to_string())
        .path("/")
        .secure(true)
        .max_age(TimeDuration::minutes(10080))
        .finish();

    cookie
}

pub fn create_expired_cookie<'c>() -> Cookie<'c> {
    let cookie_struct = CookieStruct {
        // TODO: Should this be like so?
        encoded_session_id: "This cookie done".to_string(),
    };

    let cookie = Cookie::build(COOKIE_NAME, json!(cookie_struct).to_string())
        .path("/")
        .secure(true)
        .expires(OffsetDateTime::UNIX_EPOCH)
        .finish();

    cookie
}

pub fn get_cookie_from_token(ctx: &Context<'_>) -> Option<CookieStruct> {
    let token = ctx.data::<Token>();

    match token {
        Ok(token) => {
            let c = Cookie::parse::<&str>(token.0.as_str()).unwrap();
            let (_, value) = c.name_value();
            Some(serde_json::from_str(value).expect("parsing cookie error"))
        }
        Err(e) => {
            info!("No token/cookie: {:?}", e);
            None
        }
    }
}