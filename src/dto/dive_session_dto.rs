use crate::schema::dive_sessions;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::db_query_dto::DBQueryObject;

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

// Matches the database object 1:1
#[derive(Queryable, SimpleObject)]
#[graphql(complex)]
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

#[ComplexObject]
impl DiveSessionQueryData {
    async fn dives(
        &self,
        ctx: &Context<'_>,
        db_query_dto: DBQueryObject,
        mut dive_query: DiveSessionQueryInput,
    ) -> i32 {
        42
    }
}

#[derive(InputObject)]
pub struct DiveSessionQueryInput {
    pub session_id: Option<Uuid>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub session_name: Option<String>,
    pub user_id: Uuid,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
