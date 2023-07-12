use actix_web::{cookie::ParseError as CookieParseError, error::BlockingError};
use async_graphql::Error as AsyncError;
use base64::DecodeError;
use chrono::ParseError as ChronoParseError;
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;
use redis::RedisError;
use serde_json::Error as SerdeError;
use snafu::prelude::*;
use std::{array::TryFromSliceError, num::ParseIntError};
use strum::ParseError as StrumParseError;
use uuid::Error as UuidError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum BigError {
    // ACTIX
    #[snafu(display("web::block error: {}", source))]
    ActixBlockingError { source: BlockingError },

    // COOKIE
    #[snafu(display("{source}"))]
    WrongCookieString { source: CookieParseError },

    #[snafu(display("No Cookie present: {}", error.message))]
    AsyncIncorrectCookie { error: AsyncError },

    #[snafu(display("Generic no session id: No session_id on Token"))]
    NoSessionIDOnToken,

    // SERDE
    #[snafu(display("Serde Parse Error: {}", source))]
    SerdeParseError { source: SerdeError },

    #[snafu(display("Serde Serialize Error: {}", source))]
    SerdeSerializeError { source: SerdeError },

    // Chrono
    #[snafu(display("Chrono Session: No session_id on Token"))]
    ChronoSessionError { source: ChronoParseError },

    // OTHER ASYNC
    #[snafu(display("AsyncQueryError: {}", error.message))]
    AsyncQueryError { error: AsyncError },

    // Uuid
    #[snafu(display("Uuid Error: {}", source))]
    UuidParsingerror { source: UuidError },

    // other parsing
    #[snafu(display("ParseIntError: {}", source))]
    ParseIntError { source: ParseIntError },

    #[snafu(display("StrumParseError: {}", source))]
    StrumParseError { source: StrumParseError },

    #[snafu(display("VersionParsingError"))]
    VersionParsingError,

    #[snafu(display("ParseError: {}", source))]
    ChronoParseError { source: ChronoParseError },

    // SESSION
    #[snafu(display("RedisSessionError: {}", source))]
    RedisSessionError { source: RedisError },

    // R2D2
    #[snafu(display("R2D2 Error: {}", source))]
    R2D2Error { source: R2D2Error },

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

    #[snafu(display("Incorrect DB values error: "))]
    DieselIncorrectDBValues,

    // LOGIN
    #[snafu(display("incorrect password"))]
    WrongPassword,

    // Form
    #[snafu(display("Fields not matching"))]
    FormFieldNotMatching,

    #[snafu(display("Fields not matching: {}", val))]
    FormValueNotMatching { val: String },

    // Encoding/Decoding
    #[snafu(display("DecodeError: {}", source))]
    DecodeError { source: DecodeError },

    // TryFromSlice
    #[snafu(display("TryFromSliceError: {}", source))]
    TryFromSliceError { source: TryFromSliceError },
}
