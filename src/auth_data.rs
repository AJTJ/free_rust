use serde::Serialize;

#[derive(Serialize)]
pub struct SessionData {
    id: String,
}

pub type UniversalIdType = [u8; 32];
