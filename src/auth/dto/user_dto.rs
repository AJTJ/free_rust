use crate::{
    apnea_sessions::{
        actions::get_apnea_sessions_paginated,
        dto::apnea_session_dto::{ApneaSession, ApneaSessionRetrievalData},
    },
    graphql_schema::DbPool,
    schema::users,
    utility::{
        errors::BigError,
        gql::{graphql_query::gql_query, query_dto::QueryParams},
    },
};
use actix_web::web;
use async_graphql::{
    connection::Connection, ComplexObject, Context, InputObject, OneofObject, SimpleObject,
};
use chrono::{DateTime, Utc};
// use tracing::{debug_span, event, info, instrument, span, Level};
use uuid::Uuid;

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
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
    pub is_email_verified: Option<bool>,
    pub verification_code: Option<String>,
    pub verification_code_expiry: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserCreation {
    pub username: String,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub last_login: DateTime<Utc>,
    pub is_email_verified: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

// #[derive(AsChangeset, InputObject, Clone)]
// #[diesel(table_name = users)]
// pub struct UserPasswordUpdate {
//     pub hashed_password: String,
//     pub password_salt: Vec<u8>,
// }

// This one needs to match 1:1

#[derive(Queryable, SimpleObject, Debug)]
#[graphql(complex)]
pub struct User {
    pub username: String,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub last_login: DateTime<Utc>,
    pub is_email_verified: bool,
    pub verified_date: Option<DateTime<Utc>>,
    pub verification_code: Option<String>,
    pub verification_code_expiry: Option<DateTime<Utc>>,

    // default data
    pub id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    #[graphql(skip)]
    pub archived_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub archived_by: Option<Uuid>,
}

#[ComplexObject]
impl User {
    async fn apnea_sessions(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Connection<String, ApneaSession>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user_id = self.id;

        println!("in user impl apnea sessions");

        let my_closure = move |query_params: QueryParams| {
            let query_params = query_params.clone();
            let pool_ctx = pool_ctx.clone();
            async move {
                web::block(move || {
                    let mut conn = pool_ctx.get().unwrap();

                    get_apnea_sessions_paginated(
                        &mut conn,
                        ApneaSessionRetrievalData::User(user_id),
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

#[derive(OneofObject)]
pub enum UserRetrievalData {
    Email(String),
    Id(Uuid),
}
