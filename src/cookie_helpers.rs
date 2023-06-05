use actix_web::cookie::time::{Duration as TimeDuration, OffsetDateTime};
use actix_web::cookie::Cookie;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

// COOKIE THINGS
pub const COOKIE_NAME: &str = "free_rust_cookie";

#[derive(Serialize, Deserialize)]
pub struct CookieStruct {
    pub encoded_session_id: String,
}

pub fn get_cookie<'c>(encoded_session_id: String) -> Cookie<'c> {
    let cookie_struct = CookieStruct { encoded_session_id };

    let cookie = Cookie::build(COOKIE_NAME, json!(cookie_struct).to_string())
        .path("/")
        .secure(true)
        .max_age(TimeDuration::minutes(10080))
        .finish();

    cookie
}

pub fn get_expired_cookie<'c>() -> Cookie<'c> {
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
