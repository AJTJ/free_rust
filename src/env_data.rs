#[derive(Debug)]
pub struct SharedVars {
    pub environment: String,
}

pub const DEV_ENV: &str = "DEVELOPMENT";
pub const PROD_ENV: &str = "PRODUCTION";
