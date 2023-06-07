use crate::actions::add_dive_session;
use crate::actions::add_user;
use crate::actions::get_dive_sessions;
use crate::actions::get_user_with_email;
use crate::actions::login;
use crate::actions::logout;
use crate::dto::db_query_dto;
use crate::dto::db_query_dto::DBQueryObject;
use crate::dto::dive_session_dto::DiveSessionInputData;
use crate::dto::dive_session_dto::DiveSessionQueryData;
use crate::dto::dive_session_dto::DiveSessionQueryInput;
use crate::dto::user_auth_dto::{LoginData, UserInputData, UserQueryData};
use crate::errors::ErrorEnum;
use crate::guards::LoggedInGuard;

use actix_web::error;
use actix_web::web;
use async_graphql::FieldResult;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use uuid::Uuid;

pub type DiveQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/*
   How to make it more GraphQL-y
*/

#[Object]
impl QueryRoot {
    // Purely for testing
    async fn all_users<'ctx>(&self, inc_ctx: &Context<'ctx>) -> FieldResult<Vec<UserQueryData>> {
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

    async fn user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        query_email: String,
    ) -> FieldResult<UserQueryData> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_user_with_email(&mut conn, query_email)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(user)
    }

    async fn dive_sessions<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        dive_session_input: DiveSessionQueryInput,
        db_query_dto: DBQueryObject,
    ) -> FieldResult<Vec<DiveSessionQueryData>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let dive_sessions = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions(&mut conn, dive_session_input, db_query_dto)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(dive_sessions)
    }

    // DIVE THINGS

    // async fn get_dive(&self) {}

    // Purely for testing
    #[graphql(guard = "LoggedInGuard {}")]
    async fn guarded(&self) -> bool {
        true
    }
}

#[Object]
impl MutationRoot {
    // #[graphql(guard = "LoggedInGuard {}")]
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

    // Purely for testing
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
        login(ctx, login_data.email, login_data.hashed_password).await
    }

    async fn logout(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        logout(ctx).await;
        // TODO: This could be a better return val?
        Ok(true)
    }

    // DIVE SESSION
    async fn add_session(
        &self,
        ctx: &Context<'_>,
        session_input_data: DiveSessionInputData,
    ) -> FieldResult<DiveSessionQueryData> {
        add_dive_session(ctx, session_input_data).await
    }

    // async fn get_session(&self) {}
    // async fn modify_session(&self) {}
    // async fn modify_session(&self) {}

    // // DIVES
    // async fn add_dive(&self) {}
    // async fn modify_dive(&self) {}
}
