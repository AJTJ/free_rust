#[derive(Debug)]
pub struct SharedEnvVars {
    pub environment: String,
}

pub const DEV_ENV: &str = "DEVELOPMENT";
pub const PROD_ENV: &str = "PRODUCTION";
