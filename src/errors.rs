use std::fmt::{self, Display};

use actix_web::cookie::ParseError;
use async_graphql::Error as AsyncError;
use diesel::result::Error as DieselError;
use redis::RedisError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum LoginErrorEnum {
    WrongPassword(String),
    UserNotFound(DieselError),
}

impl Display for LoginErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginErrorEnum::WrongPassword(pw) => write!(f, "incorrect password"),
            LoginErrorEnum::UserNotFound(e) => write!(f, "login error: {e}"),
        }
    }
}

impl std::error::Error for LoginErrorEnum {}

#[derive(Debug)]
pub enum DBErrors {
    QueryError(DieselError),
    UpdateError(DieselError),
}

impl Display for DBErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBErrors::QueryError(e) => write!(f, "Query error: {e}"),
            DBErrors::UpdateError(e) => write!(f, "Update error: {e}"),
        }
    }
}

impl std::error::Error for DBErrors {}

#[derive(Debug)]
pub enum SessionCookieErrors {
    CookieError(CookieError),
    SessionError(RedisError),
}

impl Display for SessionCookieErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionCookieErrors::CookieError(e) => write!(f, "CookieError: {e}"),
            SessionCookieErrors::SessionError(e) => write!(f, "SessionError: {e}"),
        }
    }
}

impl std::error::Error for SessionCookieErrors {}

// TEMPLATE
// #[derive(Debug)]
// pub enum E {}

// impl Display for E {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {}
//     }
// }

// impl std::error::Error for E {}

#[derive(Debug)]
pub enum CookieError {
    WrongCookieString(ParseError),
    NoCookie(AsyncError),
    ParsingCookieVal(SerdeError),
}

impl Display for CookieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CookieError::WrongCookieString(e) => write!(f, "Parsed cookie doesn't match: {e}"),
            CookieError::NoCookie(e) => write!(f, "No Cookie present: {}", e.message),
            CookieError::ParsingCookieVal(e) => write!(f, "Error parsing cookie val: {e}"),
        }
    }
}

impl std::error::Error for CookieError {}
