use crate::{actions::get_dive_sessions_by_user, graphql_schema::DbPool, schema::users};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(InputObject)]
pub struct FooInput {}

#[derive(AsChangeset, InputObject)]
#[diesel(table_name = users)]
pub struct FooUpdate {}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct FooCreation {
    // partial default data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
#[graphql(complex)]
pub struct Foo {
    // default data
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[derive(InputObject)]
pub struct FooQueryParams {}

// not sure this one is necessary
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct FooOutput {}

impl From<User> for UserOutput {
    fn from(val: User) -> Self {
        UserOutput {
            id: val.id.into(),
            username: val.username,
            email: val.email,
            last_login: val.last_login,

            created_at: val.created_at,
            updated_at: val.updated_at,
            is_active: val.is_active,
        }
    }
}

#[ComplexObject]
impl UserOutput {
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        // this needs to be mut
        dive_session_query: Option<DiveSessionFilter>,
        db_query_dto: Option<QueryParams>,
    ) -> FieldResult<Vec<DiveSession>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let user_id: Uuid =
            Uuid::parse_str(&self.id).map_err(|e| BigError::UuidParsingerror { source: e })?;
        let dive_sessions = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, &user_id, dive_session_query, db_query_dto)
        })
        .await
        .expect("error in dive sessions web::block")
        .expect("error in another loading dive sessions");

        Ok(dive_sessions)
    }
}
