use async_graphql::{InputObject, SimpleObject};
use chrono::{NaiveDateTime, NaiveTime};
use uuid::Uuid;

use crate::schema::dives;

#[derive(InputObject)]
pub struct DiveInput {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,
}

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = dives)]
pub struct DiveUpdate {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,

    pub id: Uuid,
    pub is_active: Option<bool>,
}

#[derive(Insertable, InputObject)]
#[diesel(table_name = dives)]
pub struct DiveCreation {
    pub id: Uuid,
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// Matches the database object order 1:1
#[derive(Queryable, SimpleObject)]
pub struct DiveQuery {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,

    pub session_id: Uuid,
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[derive(InputObject)]
pub struct DiveQueryInput {
    pub dive_id: Option<Uuid>,
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<NaiveTime>,
    pub dive_name: Option<String>,
    pub dive_session: Option<Uuid>,
    pub user_id: Option<Uuid>,

    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
