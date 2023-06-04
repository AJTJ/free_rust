use chrono::NaiveDateTime;
use redis::{from_redis_value, Client, FromRedisValue, RedisResult, ToRedisArgs};
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

pub const COOKIE_NAME: &str = "free_rust_cookie";
pub type SharedRedisType = Arc<Mutex<Client>>;

#[derive(Serialize, Deserialize)]
pub struct SessionKeyValue {
    // currently using an encoded UniversalIdType
    // why? Why not.
    key: String,
    value: SessionData,
}

impl ToRedisArgs for SessionKeyValue {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg_fmt(
            serde_json::to_string(self).expect("Can't serialize SessionKeyValue as string"),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionData {
    pub user_id: Uuid,
    pub expiry: NaiveDateTime,
}

impl ToRedisArgs for SessionData {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg_fmt(
            serde_json::to_string(self).expect("Can't serialize SessionData as string"),
        )
    }
}

impl FromRedisValue for SessionData {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let redis_value: String = from_redis_value(&v).expect("from_redis_value to String failing");
        let session_data: SessionData =
            serde_json::from_str(&redis_value).expect("redis to SessionData failing");
        RedisResult::Ok(session_data)
    }
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
