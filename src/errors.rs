use std::fmt::{self, Display};

use diesel::result::Error;
// use snafu::prelude::*;

// #[derive(Debug, Snafu)]
// pub enum CustomError {
//     #[snafu(display("Wrong password: {pw}"))]
//     WrongPassword { pw: String },
// }

// type Result<T, E = CustomError> = std::result::Result<T, E>;
// fn example(pw: String) -> Result<()> {
//     ensure!(pw == "dog".to_string(), WrongPasswordSnafu { pw });
//     Ok(())
// }

// #[derive(Debug)]
// pub struct WrongPassword;

// impl Error for WrongPassword {}

// impl fmt::Display for WrongPassword {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Wrong password used.")
//     }
// }

// // error1.rs

// #[derive(Debug)]
// pub struct MyError {
//     details: String,
// }

// impl MyError {
//     fn new(msg: &str) -> MyError {
//         MyError {
//             details: msg.to_string(),
//         }
//     }
// }

// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.details)
//     }
// }

// impl Error for MyError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }

// // a test function that returns our error result
// fn raises_my_error(yes: bool) -> Result<(), MyError> {
//     if yes {
//         Err(MyError::new("borked"))
//     } else {
//         Ok(())
//     }
// }

#[derive(Debug)]
pub enum LoginErrorEnum {
    WrongPassword(String),
    UserNotFound(Error),
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
    QueryError(Error),
    UpdateError(Error),
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
