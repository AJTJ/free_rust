use crate::schema::dive_sessions;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::{NaiveDateTime, NaiveTime};
use uuid::Uuid;

// Matches the database object 1:1
#[derive(Queryable, SimpleObject)]
pub struct DiveQueryData {
    pub id: i32,
    pub dive_id: Uuid,
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<NaiveTime>,
    pub dive_name: Option<String>,
    pub dive_session: Uuid,
    pub user_id: Uuid,

    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

pub struct DiveQueryInput {
    pub dive_id: Uuid,
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<NaiveTime>,
    pub dive_name: Option<String>,
    pub dive_session: Uuid,
    pub user_id: Option<Uuid>,

    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
