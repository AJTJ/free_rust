use crate::actions::add_to_session;
use crate::actions::add_user;
use crate::actions::get_user_with_email;
use crate::actions::login;
use crate::actions::logout;
use crate::auth_data::SessionData;
use crate::data::LoginData;
use crate::data::UserInputData;
use crate::data::UserQueryData;
use crate::errors::ErrorEnum;
use actix_web::error;
use actix_web::web;
use async_graphql::FieldResult;
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use chrono::Duration;
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use tracing::info;

pub type DiveQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

#[derive(Debug, Clone, SimpleObject)]
pub struct TestObject {
    id: ID,
    val: String,
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Identity {
    pub id: Option<String>,
}

#[Object]
impl QueryRoot {
    async fn get_all_users<'ctx>(
        &self,
        inc_ctx: &Context<'ctx>,
    ) -> FieldResult<Vec<UserQueryData>> {
        let pool_ctx = inc_ctx.data_unchecked::<DbPool>().clone();

        let all_users = web::block(move || {
            let pool = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            users
                .load::<UserQueryData>(&pool)
                .expect("loading all users")
        })
        .await?;

        Ok(all_users)
    }

    async fn get_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        query_email: String,
    ) -> FieldResult<UserQueryData> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user = web::block(move || {
            let mut pool = pool_ctx.get().unwrap();
            get_user_with_email(&mut pool, query_email)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();
        Ok(user)
    }

    // DIVE SESSION

    // async fn get_session(&self) {
    //     unimplemented!()
    // }
}

#[Object]
impl MutationRoot {
    async fn insert_user(
        &self,
        ctx: &Context<'_>,
        user_data: UserInputData,
    ) -> FieldResult<UserQueryData> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            add_user(&mut conn, user_data)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(user)
    }

    async fn delete_all_users(&self, ctx: &Context<'_>) -> FieldResult<usize> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let deleted = web::block(move || {
            let conn = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            diesel::delete(users).execute(&conn).unwrap()
        })
        .await?;

        Ok(deleted)
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        login_data: LoginData,
    ) -> Result<UserQueryData, ErrorEnum> {
        let return_user = login(ctx, login_data.email, login_data.hashed_password).await;

        if let Ok(user) = return_user {
            add_to_session(
                ctx,
                SessionData {
                    user_id: user.user_id,
                    expiry: Utc::now().naive_utc() + Duration::minutes(10080),
                },
                // encoded_session_id.clone(),
                "123".to_string(),
            );
        }

        return_user
    }

    async fn logout(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        Ok(logout(ctx))
    }

    // if let Ok(user_data) = login_result {
    //     add_to_session(
    //         ctx,
    //         SessionData {
    //             user_id: user_data.user_id,
    //             expiry: Utc::now().naive_utc() + Duration::minutes(10080),
    //         },
    //         encoded_session_id.clone(),
    //     )
    //     .await;
    // }

    // // DIVE SESSION
    // async fn add_session(&self) {}
    // async fn modify_session(&self) {}

    // // DIVES
    // async fn add_dive(&self) {}
    // async fn modify_dive(&self) {}
}
