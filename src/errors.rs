use std::{error::Error, fmt};

#[derive(Debug)]
pub struct WrongPassword;

impl Error for WrongPassword {}

impl fmt::Display for WrongPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong password used.")
    }
}

// error1.rs

#[derive(Debug)]
pub struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

// a test function that returns our error result
fn raises_my_error(yes: bool) -> Result<(), MyError> {
    if yes {
        Err(MyError::new("borked"))
    } else {
        Ok(())
    }
}
