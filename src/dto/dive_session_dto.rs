use crate::schema::dive_sessions;
use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(InputObject)]
pub struct DiveSessionInputData {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,
}

#[derive(Insertable, InputObject)]
#[table_name = "dive_sessions"]
pub struct DiveSessionCreationData {
    pub session_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
pub struct DiveSessionQueryData {
    pub id: i32,
    pub session_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}
