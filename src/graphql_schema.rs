use crate::actions::add_dive;
use crate::actions::add_dive_session;
use crate::actions::add_logger;
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
use crate::dto::dive_dto::Dive;
use crate::dto::dive_dto::DiveFilter;
use crate::dto::dive_dto::DiveInput;
use crate::dto::dive_dto::DiveUpdate;
use crate::dto::dive_session_dto::DiveSession;
use crate::dto::dive_session_dto::DiveSessionFilter;
use crate::dto::dive_session_dto::DiveSessionInput;
use crate::dto::dive_session_dto::DiveSessionUpdate;
use crate::dto::log_dto::Log;
use crate::dto::logger_entries_dto::LoggerEntry;
use crate::dto::loggers_dto::Logger;
use crate::dto::loggers_dto::LoggerInput;
use crate::dto::query_dto::QueryParams;
use crate::dto::user_dto::UserOutput;
use crate::dto::user_dto::{User, UserInput};
use crate::errors::BigError;
use crate::guards::{DevelopmentGuard, LoggedInGuard};
use crate::helpers::form_helper::UserFormInput;
use actix_web::web;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use rand::prelude::*;
use uuid::Uuid;

pub type DiveQLSchema = Schema<Query, Mutation, EmptySubscription>;
pub struct Query;
pub struct Mutation;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[Object]
impl Query {
    // UNGUARDED - for testing
    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn all_users(&self, ctx: &Context<'_>) -> Result<Vec<User>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            users.load::<User>(&mut conn)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn user(&self, ctx: &Context<'_>, email: String) -> Result<User, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_user_with_email(&mut conn, email)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        dive_session_input: Option<DiveSessionFilter>,
        db_query_dto: Option<QueryParams>,
    ) -> Result<Vec<DiveSession>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user_id = get_user_id_from_token_and_session(ctx).await?;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, &user_id, dive_session_input, db_query_dto)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn dives(
        &self,
        ctx: &Context<'_>,
        dive_input: Option<DiveFilter>,
        db_query_dto: Option<QueryParams>,
    ) -> Result<Vec<Dive>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user_id = get_user_id_from_token_and_session(ctx).await?;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dives_by_user(&mut conn, user_id, dive_input, db_query_dto)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
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
        .map_err(|e| BigError::BlockingError { source: e })?
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
        .map_err(|e| BigError::BlockingError { source: e })?
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
        .map_err(|e| BigError::BlockingError { source: e })?
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
impl Mutation {
    // Must be UNGUARDED?
    async fn insert_user(&self, ctx: &Context<'_>, user_data: UserInput) -> Result<User, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            insert_user(&mut conn, user_data)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselInsertError { source: e })
    }

    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn delete_all_users(&self, ctx: &Context<'_>) -> Result<usize, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::users;
            diesel::delete(users).execute(&mut conn)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselDeleteError { source: e })
    }

    // AUTH
    // Must be UNGUARDED?
    async fn login(&self, ctx: &Context<'_>, login_data: Login) -> Result<UserOutput, BigError> {
        login(ctx, login_data.email, login_data.password).await
    }

    // Should be guarded eventually
    // #[graphql(guard = "LoggedInGuard::new()")]
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, BigError> {
        logout(ctx).await
    }

    // DIVE SESSION
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn add_dive_session(
        &self,
        ctx: &Context<'_>,
        dive_session_input: DiveSessionInput,
    ) -> Result<DiveSession, BigError> {
        add_dive_session(ctx, dive_session_input).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn update_dive_session(
        &self,
        ctx: &Context<'_>,
        dive_session_update: DiveSessionUpdate,
    ) -> Result<DiveSession, BigError> {
        update_dive_session(ctx, dive_session_update).await
    }

    // for testing
    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn delete_all_dive_sessions(&self, ctx: &Context<'_>) -> Result<usize, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::dive_sessions::dsl::dive_sessions;
            diesel::delete(dive_sessions).execute(&mut conn)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselDeleteError { source: e })
    }

    // DIVES
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn add_dive(
        &self,
        ctx: &Context<'_>,
        dive_session_id: Uuid,
        dive_input: DiveInput,
    ) -> Result<Dive, BigError> {
        add_dive(ctx, dive_session_id, dive_input).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn update_dive(
        &self,
        ctx: &Context<'_>,
        dive_update: DiveUpdate,
    ) -> Result<Dive, BigError> {
        update_dive(ctx, dive_update).await
    }

    // TODOS
    #[graphql(guard = "LoggedInGuard::new()")]

    async fn add_logger(
        &self,
        ctx: &Context<'_>,
        logger_data: LoggerInput,
        user_form_input: UserFormInput,
    ) -> Result<Logger, BigError> {
        add_logger(ctx, logger_data, user_form_input).await
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
    async fn delete_all_dives(&self, ctx: &Context<'_>) -> Result<usize, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::dives::dsl::dives;
            diesel::delete(dives).execute(&mut conn)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselDeleteError { source: e })
    }
}
