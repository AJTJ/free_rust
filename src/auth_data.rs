use chrono::NaiveDateTime;
use send_wrapper::SendWrapper;
use serde::Serialize;
use std::ops::Deref;
use uuid::Uuid;

#[derive(Serialize)]
pub struct SessionData {
    pub user_id: Uuid,
    pub expiry: NaiveDateTime,
}

#[derive(Serialize)]
pub struct SessionKeyValue {
    key: String,
    value: SessionData,
}

pub type UniversalIdType = [u8; 32];

// TODO: Is this baaad?
#[derive(Clone, Debug)]
pub struct Shared<T>(pub Option<SendWrapper<T>>);

impl<T> Shared<T> {
    pub fn new(v: T) -> Self {
        Self(Some(SendWrapper::new(v)))
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0.as_deref().clone().unwrap()
    }
}
