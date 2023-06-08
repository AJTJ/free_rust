use chrono::NaiveDateTime;
use redis::{from_redis_value, Client, FromRedisValue, RedisResult, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::string::String;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub type UniversalIdType = [u8; 32];

// REDIS THINGS
pub type SharedRedisType = Arc<Mutex<Client>>;

#[derive(Serialize, Deserialize)]
pub struct RedisKeyType(String);

impl ToRedisArgs for RedisKeyType {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("can't serialize the RedisKeyType"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionKeyValue {
    // currently using an ENCODED UniversalIdType
    // why? Why not.
    key: RedisKeyType,
    value: SessionData,
}

pub type RedisSetType = (String, SessionData, bool);

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
