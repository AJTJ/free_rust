use crate::actions::add_dive;
use crate::actions::add_dive_session;
use crate::actions::get_dive_sessions_by_user;
use crate::actions::get_dives_by_user;
use crate::actions::get_logger_entries_by_logger;
use crate::actions::get_loggers_from_user_id;
use crate::actions::get_logs_from_user_id;
use crate::actions::get_user_id_from_token_and_session;
use crate::actions::get_user_with_email;
use crate::actions::insert_user;
use crate::actions::login;
use crate::actions::logout;
use crate::actions::update_dive;
use crate::actions::update_dive_session;
use crate::dto::auth_dto::LoginData;
use crate::dto::db_query_dto::DBQueryObject;
use crate::dto::dive_dto::DiveInputData;
use crate::dto::dive_dto::DiveModificationData;
use crate::dto::dive_dto::DiveQueryData;
use crate::dto::dive_dto::DiveQueryInput;
use crate::dto::dive_session_dto::DiveSessionInputData;
use crate::dto::dive_session_dto::DiveSessionModificationData;
use crate::dto::dive_session_dto::DiveSessionQueryData;
use crate::dto::dive_session_dto::DiveSessionQueryInput;
use crate::dto::log_dto::LogData;
use crate::dto::loggers_dto::LoggerData;
use crate::dto::loggers_dto::LoggerEntryData;
use crate::dto::user_dto::UserQueryDataOutput;
use crate::dto::user_dto::{UserInputData, UserQueryData};
use crate::errors::BigError;
use crate::guards::{DevelopmentGuard, LoggedInGuard};
use rand::prelude::*;

use actix_web::error;
use actix_web::web;
use async_graphql::FieldResult;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use tracing::info;
use uuid::Uuid;

pub type DiveQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[Object]
impl QueryRoot {
    // UNGUARDED - for testing
    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn all_users<'ctx>(&self, inc_ctx: &Context<'ctx>) -> FieldResult<Vec<UserQueryData>> {
        let pool_ctx = inc_ctx.data_unchecked::<DbPool>().clone();

        let all_users = web::block(move || {
            let mut pool = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            users
                .load::<UserQueryData>(&mut pool)
                .expect("loading all users")
        })
        .await?;

        Ok(all_users)
    }

    #[graphql(guard = "LoggedInGuard::new()")]
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

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        dive_session_input: Option<DiveSessionQueryInput>,
        db_query_dto: Option<DBQueryObject>,
    ) -> FieldResult<Vec<DiveSessionQueryData>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let user_id = get_user_id_from_token_and_session(ctx).await?;

        let dive_sessions = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, &user_id, dive_session_input, db_query_dto)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(dive_sessions)
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn dives(
        &self,
        ctx: &Context<'_>,
        dive_input: Option<DiveQueryInput>,
        db_query_dto: Option<DBQueryObject>,
    ) -> FieldResult<Vec<DiveQueryData>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let user_id = get_user_id_from_token_and_session(ctx).await?;

        let dives = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dives_by_user(&mut conn, user_id, dive_input, db_query_dto)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();
        Ok(dives)
    }

    // LOGGERS

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn loggers(&self, ctx: &Context<'_>) -> Result<Vec<LoggerData>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await.unwrap();
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_loggers_from_user_id(&mut conn, user_id, None)
        })
        .await
        .unwrap()
        .map_err(|e| BigError::QueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn logger_entries(
        &self,
        ctx: &Context<'_>,
        logger_id: Uuid,
    ) -> Result<Vec<LoggerEntryData>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await.unwrap();
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_logger_entries_by_logger(&mut conn, &logger_id, &user_id, None)
        })
        .await
        .unwrap()
        .map_err(|e| BigError::QueryError { source: e })
    }

    // LOGS

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn logs(&self, ctx: &Context<'_>) -> Result<Vec<LogData>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await.unwrap();
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_logs_from_user_id(&mut conn, user_id, None)
        })
        .await
        .unwrap()
        .map_err(|e| BigError::QueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn guarded_route(&self, ctx: &Context<'_>) -> f64 {
        // Ok("Made it".to_string())
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        y
    }
}

#[Object]
impl MutationRoot {
    // Must be UNGUARDED?
    async fn insert_user(
        &self,
        ctx: &Context<'_>,
        user_data: UserInputData,
    ) -> FieldResult<UserQueryData> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            insert_user(&mut conn, user_data)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(user)
    }

    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn delete_all_users(&self, ctx: &Context<'_>) -> FieldResult<usize> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let deleted = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::users;
            diesel::delete(users)
                .execute(&mut conn)
                .expect("problem deleting users")
        })
        .await?;

        Ok(deleted)
    }

    // AUTH
    // Must be UNGUARDED?
    async fn login(
        &self,
        ctx: &Context<'_>,
        login_data: LoginData,
    ) -> Result<UserQueryDataOutput, BigError> {
        login(ctx, login_data.email, login_data.password).await
    }

    // Should be guarded eventually
    // #[graphql(guard = "LoggedInGuard::new()")]
    async fn logout(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        logout(ctx).await?;
        // TODO: This could be a better return val?
        info!("logout done");
        Ok(true)
    }

    // DIVE SESSION
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn add_dive_session(
        &self,
        ctx: &Context<'_>,
        session_input_data: DiveSessionInputData,
    ) -> FieldResult<DiveSessionQueryData> {
        add_dive_session(ctx, session_input_data).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn update_dive_session(
        &self,
        ctx: &Context<'_>,
        session_input_data: DiveSessionModificationData,
    ) -> Result<DiveSessionQueryData, BigError> {
        update_dive_session(ctx, session_input_data).await
    }

    // for testing
    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn delete_all_dive_sessions(&self, ctx: &Context<'_>) -> FieldResult<usize> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let deleted = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::dive_sessions::dsl::dive_sessions;
            diesel::delete(dive_sessions)
                .execute(&mut conn)
                .expect("problem deleting dive sessions")
        })
        .await?;

        Ok(deleted)
    }

    // DIVES
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn add_dive(
        &self,
        ctx: &Context<'_>,
        dive_session_id: Uuid,
        dive_data: DiveInputData,
    ) -> FieldResult<DiveQueryData> {
        add_dive(ctx, dive_session_id, dive_data).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn update_dive(
        &self,
        ctx: &Context<'_>,
        dive_mod_data: DiveModificationData,
    ) -> FieldResult<DiveQueryData> {
        let updated_dive = update_dive(ctx, dive_mod_data).await;
        Ok(updated_dive)
    }

    // TODOS
    // LOGGER STUFF
    // add_logger() {}
    // edit_logger() {}
    // delete_logger() {}

    // LOGGER_INPUT STUFF
    // add_logger_input() {}
    // edit_logger_input() {}
    // delete_logger_input() {}

    // LOG STUFF
    // add_log() {}
    // edit_log() {}
    // delete_log() {}

    // LOG_INPUT STUFF
    // add_log_input() {}
    // edit_log_input() {}
    // delete_log_input() {}

    //for testing only
    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn delete_all_dives(&self, ctx: &Context<'_>) -> FieldResult<usize> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let deleted = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::dives::dsl::dives;
            diesel::delete(dives)
                .execute(&mut conn)
                .expect("problem deleting dives")
        })
        .await?;

        Ok(deleted)
    }
}
