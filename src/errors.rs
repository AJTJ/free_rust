use std::{
    fmt::{self, Display},
    io::Error,
    num::ParseIntError,
    result::Er,
};

use actix_web::{cookie::ParseError as CookieParseError, error::BlockingError, HttpResponse};
use async_graphql::Error as AsyncError;
use chrono::ParseError as ChronoParseError;
use diesel::result::Error as DieselError;
use redis::RedisError;
use serde_json::Error as SerdeError;
use snafu::prelude::*;
use thiserror::Error as ThisError;
use uuid::Error as UuidError;

#[derive(Debug, Snafu, Clone)]
pub enum BigError {
    // ACTIX
    #[snafu(display("web::block error: {}", source))]
    ActixBlockingError { source: BlockingError },

    // COOKIE
    #[snafu(display("{source}"))]
    WrongCookieString { source: CookieParseError },

    #[snafu(display("No Cookie present: {}", error.message))]
    IncorrectCookie { error: AsyncError },

    #[snafu(display("Error parsing cookie val: {}", source))]
    SerdeParsingCookieVal { source: SerdeError },

    #[snafu(display("No session_id on Token"))]
    NoSessionIDOnToken,

    // Uuid
    #[snafu(display("Uuid Error: {}", source))]
    UuidParsingerror { source: UuidError },

    // other parsing
    #[snafu(display("ParseIntError: {}", source))]
    ParseIntError { source: ParseIntError },

    #[snafu(display("ParseError: {}", source))]
    ChronoParseError { source: ChronoParseError },

    // SESSION
    #[snafu(display("RedisSessionError: {}", source))]
    RedisSessionError { source: RedisError },

    // DB/DIESEL
    #[snafu(display("QueryError: {}", source))]
    DieselQueryError { source: DieselError },

    #[snafu(display("UpdateError: {}", source))]
    DieselUpdateError { source: DieselError },

    #[snafu(display("User Not Found: {}", source))]
    DieselUserNotFound { source: DieselError },

    #[snafu(display("Insert Error: {}", source))]
    DieselInsertError { source: DieselError },

    #[snafu(display("Delete Error: {}", source))]
    DieselDeleteError { source: DieselError },

    // LOGIN
    #[snafu(display("incorrect password"))]
    WrongPassword,

    // Form
    #[snafu(display("Fields not matching"))]
    FormFieldNotMatching,

    #[snafu(display("Fields not matching"))]
    FormValueNotMatching,
}

#[derive(Debug, Snafu, Clone)]
pub enum DieselErrors {
    #[snafu(display("QueryError: {}", source))]
    QueryError { source: DieselError },
}

// #[derive(Debug)]
// pub enum LoginErrorEnum {
//     WrongPassword(String),
//     UserNotFound(DieselError),
// }

// impl Display for LoginErrorEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             LoginErrorEnum::WrongPassword(pw) => write!(f, "incorrect password"),
//             LoginErrorEnum::UserNotFound(e) => write!(f, "login error: {e}"),
//         }
//     }
// }

// impl std::error::Error for LoginErrorEnum {}

// #[derive(Debug)]
// pub enum DBErrors {
//     QueryError(DieselError),
//     UpdateError(DieselError),
// }

// impl Display for DBErrors {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             DBErrors::QueryError(e) => write!(f, "Query error: {e}"),
//             DBErrors::UpdateError(e) => write!(f, "Update error: {e}"),
//         }
//     }
// }

// impl std::error::Error for DBErrors {}

// #[derive(Debug)]
// pub enum SessionCookieErrors {
//     CookieError(CookieError),
//     SessionError(RedisError),
// }

// impl Display for SessionCookieErrors {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             SessionCookieErrors::CookieError(e) => write!(f, "CookieError: {e}"),
//             SessionCookieErrors::SessionError(e) => write!(f, "SessionError: {e}"),
//         }
//     }
// }

// impl std::error::Error for SessionCookieErrors {}

// TEMPLATE
// #[derive(Debug)]
// pub enum E {}

// impl Display for E {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {}
//     }
// }

// impl std::error::Error for E {}

// #[derive(Debug)]
// pub enum CookieError {
//     WrongCookieString(ParseError),
//     NoCookie(AsyncError),
//     ParsingCookieVal(SerdeError),
// }

// impl Display for CookieError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CookieError::WrongCookieString(e) => write!(f, "Parsed cookie doesn't match: {e}"),
//             CookieError::NoCookie(e) => write!(f, "No Cookie present: {}", e.message),
//             CookieError::ParsingCookieVal(e) => write!(f, "Error parsing cookie val: {e}"),
//         }
//     }
// }

// impl std::error::Error for CookieError {}
