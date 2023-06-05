use crate::schema::dive_sessions;
use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Insertable, InputObject)]
#[table_name = "dive_sessions"]
pub struct SessionCreationData {
    pub session_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
pub struct DiveSessionQueryData {
    pub id: i32,
    pub session_id: Uuid,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub session_name: Option<String>,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
