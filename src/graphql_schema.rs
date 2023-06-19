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
use crate::dto::auth_dto::Login;
use crate::dto::db_query_dto::DBQueryParams;
use crate::dto::dive_dto::DiveInput;
use crate::dto::dive_dto::DiveQuery;
use crate::dto::dive_dto::DiveQueryInput;
use crate::dto::dive_dto::DiveUpdate;
use crate::dto::dive_session_dto::DiveSessionInput;
use crate::dto::dive_session_dto::DiveSessionQuery;
use crate::dto::dive_session_dto::DiveSessionQueryParams;
use crate::dto::dive_session_dto::DiveSessionUpdate;
use crate::dto::log_dto::Log;
use crate::dto::loggers_dto::Logger;
use crate::dto::loggers_dto::LoggerEntry;
use crate::dto::user_dto::UserQueryOutput;
use crate::dto::user_dto::{UserInput, UserQuery};
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
    async fn all_users<'ctx>(&self, inc_ctx: &Context<'ctx>) -> FieldResult<Vec<UserQuery>> {
        let pool_ctx = inc_ctx.data_unchecked::<DbPool>().clone();

        let all_users = web::block(move || {
            let mut pool = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            users
                .load::<UserQuery>(&mut pool)
                .expect("loading all users")
        })
        .await?;

        Ok(all_users)
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, query_email: String) -> FieldResult<UserQuery> {
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
        dive_session_input: Option<DiveSessionQueryParams>,
        db_query_dto: Option<DBQueryParams>,
    ) -> FieldResult<Vec<DiveSessionQuery>> {
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
        db_query_dto: Option<DBQueryParams>,
    ) -> FieldResult<Vec<DiveQuery>> {
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
    async fn loggers(&self, ctx: &Context<'_>) -> Result<Vec<Logger>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_loggers_from_user_id(&mut conn, user_id, None)
        })
        .await
        .unwrap()
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn logger_entries(
        &self,
        ctx: &Context<'_>,
        logger_id: Uuid,
    ) -> Result<Vec<LoggerEntry>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_logger_entries_by_logger(&mut conn, &logger_id, &user_id, None)
        })
        .await
        .unwrap()
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    // LOGS

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn logs(&self, ctx: &Context<'_>) -> Result<Vec<Log>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_logs_from_user_id(&mut conn, user_id, None)
        })
        .await
        .unwrap()
        .map_err(|e| BigError::DieselQueryError { source: e })
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
    async fn insert_user(&self, ctx: &Context<'_>, user_data: UserInput) -> FieldResult<UserQuery> {
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
        login_data: Login,
    ) -> Result<UserQueryOutput, BigError> {
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
        session_input_data: DiveSessionInput,
    ) -> Result<DiveSessionQuery, BigError> {
        add_dive_session(ctx, session_input_data).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn update_dive_session(
        &self,
        ctx: &Context<'_>,
        session_input_data: DiveSessionUpdate,
    ) -> Result<DiveSessionQuery, BigError> {
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
        dive_data: DiveInput,
    ) -> Result<DiveQuery, BigError> {
        add_dive(ctx, dive_session_id, dive_data).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn update_dive(
        &self,
        ctx: &Context<'_>,
        dive_mod_data: DiveUpdate,
    ) -> FieldResult<DiveQuery> {
        let updated_dive = update_dive(ctx, dive_mod_data).await;
        Ok(updated_dive)
    }

    // TODOS
    #[graphql(guard = "LoggedInGuard::new()")]

    async fn add_logger(&self, ctx: &Context<'_>, logger_input: i32) -> i32 {
        // add_logger()
        4
    }
    // update_logger() {}
    // delete_logger() {}

    // LOGGER_INPUT STUFF
    // add_logger_input() {}
    // update_logger_input() {}
    // delete_logger_input() {}

    // LOG STUFF
    // add_log() {}
    // update_log() {}
    // delete_log() {}

    // LOG_INPUT STUFF
    // add_log_input() {}
    // update_log_input() {}
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
