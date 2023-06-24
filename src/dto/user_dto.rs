use crate::{
    actions::{get_completed_forms_by_user_id, get_dive_sessions_by_user},
    errors::BigError,
    graphql_query::gql_query,
    graphql_schema::DbPool,
    schema::users,
};
use actix_web::web;
use async_graphql::{connection::Connection, ComplexObject, Context, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    dive_session_dto::{DiveSession, DiveSessionFilter},
    query_dto::QueryParams,
};

#[derive(Clone, InputObject)]
pub struct UserInput {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = users)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserCreation {
    pub username: String,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub last_login: NaiveDateTime,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
#[graphql(complex)]
pub struct User {
    pub username: String,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub last_login: NaiveDateTime,

    // default data
    pub id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    #[graphql(skip)]
    pub archived_at: Option<NaiveDateTime>,
    #[graphql(skip)]
    pub archived_by: Option<Uuid>,
}

#[ComplexObject]
impl User {
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        dive_session_filter: Option<DiveSessionFilter>,
        query_params: QueryParams,
    ) -> Result<Connection<String, DiveSession>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user_id = self.id;

        let my_closure = move |query_params: QueryParams| {
            let query_params = query_params.clone();
            let dive_session_filter = dive_session_filter.clone();
            let pool_ctx = pool_ctx.clone();
            async move {
                web::block(move || {
                    let mut conn = pool_ctx.get().unwrap();
                    get_dive_sessions_by_user(
                        &mut conn,
                        &user_id,
                        dive_session_filter,
                        query_params,
                    )
                })
                .await
                .map_err(|e| BigError::ActixBlockingError { source: e })?
            }
        };

        let query_response = gql_query(query_params, &my_closure).await;
        query_response.map_err(|e| BigError::AsyncQueryError { error: e })
    }
}
