use crate::actions::add_dive;
use crate::actions::add_dive_session;
use crate::actions::add_form;
use crate::actions::get_completed_forms_by_user_id;
use crate::actions::get_dive_sessions_by_user;
use crate::actions::get_dives_by_user;
use crate::actions::get_form_fields_by_form;
use crate::actions::get_forms_by_user_id;
use crate::actions::get_user_id_from_token_and_session;
use crate::actions::get_user_with_email;
use crate::actions::insert_completed_form;
use crate::actions::insert_user;
use crate::actions::login;
use crate::actions::logout;
use crate::actions::update_dive;
use crate::actions::update_dive_session;
use crate::dto::auth_dto::Login;
use crate::dto::completed_form_dto::CompletedForm;
use crate::dto::completed_form_dto::CompletedFormInput;
use crate::dto::completed_form_dto::CompletedFormOutput;
use crate::dto::completed_form_field_dto::CompletedFormField;
use crate::dto::dive_dto::Dive;
use crate::dto::dive_dto::DiveFilter;
use crate::dto::dive_dto::DiveInput;
use crate::dto::dive_dto::DiveUpdate;
use crate::dto::dive_session_dto::DiveSession;
use crate::dto::dive_session_dto::DiveSessionFilter;
use crate::dto::dive_session_dto::DiveSessionInput;
use crate::dto::dive_session_dto::DiveSessionUpdate;
use crate::dto::form_dto::Form;
use crate::dto::form_dto::FormInput;
use crate::dto::form_dto::FormOutput;
use crate::dto::form_field_dto::FormField;
use crate::dto::query_dto::QueryParams;
use crate::dto::user_dto::{User, UserInput};
use crate::errors::BigError;
use crate::guards::{DevelopmentGuard, LoggedInGuard};
use actix_web::web;
use async_graphql::*;
use async_graphql::{types::connection::*, Context, EmptySubscription, Object, Schema};
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

    // async fn numbers(
    //     &self,
    //     after: Option<String>,
    //     before: Option<String>,
    //     first: Option<i32>,
    //     last: Option<i32>,
    // ) -> Result<Connection<usize, i32, EmptyFields, EmptyFields>> {
    //     query(
    //         after,
    //         before,
    //         first,
    //         last,
    //         |after, before, first, last| async move {
    //             let mut start = after.map(|after| after + 1).unwrap_or(0);
    //             let mut end = before.unwrap_or(10000);
    //             if let Some(first) = first {
    //                 end = (start + first).min(end);
    //             }
    //             if let Some(last) = last {
    //                 start = if last > end - start { end } else { end - last };
    //             }
    //             let mut connection = Connection::new(start > 0, end < 10000);
    //             connection.edges.extend(
    //                 (start..end)
    //                     .into_iter()
    //                     .map(|n| Edge::with_additional_fields(n, n as i32, EmptyFields)),
    //             );
    //             Ok::<_, async_graphql::Error>(connection)
    //         },
    //     )
    //     .await
    // }

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

        // CLOSURE IDEAS
        // let my_closure = async move |after: Option<String>,
        //                              first: Option<usize>|
        //             -> Result<Vec<DiveSession>, BigError> {
        //     web::block(move || {
        //         let mut conn = pool_ctx.get().unwrap();
        //         get_dive_sessions_by_user(&mut conn, &user_id, dive_session_input, db_query_dto)
        //     })
        //     .await
        //     .map_err(|e| BigError::BlockingError { source: e })?
        //     .map_err(|e| BigError::DieselQueryError { source: e })
        // };

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, &user_id, dive_session_input, db_query_dto)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    // .map(|dv| dv.into_iter().map(DiveSession::from).collect())

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

    // FORMS

    // TODO: Should this return the database obejct?
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn forms(&self, ctx: &Context<'_>) -> Result<Vec<Form>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_forms_by_user_id(&mut conn, user_id, None)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn form_fields(
        &self,
        ctx: &Context<'_>,
        logger_id: Uuid,
    ) -> Result<Vec<FormField>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_form_fields_by_form(&mut conn, &logger_id, &user_id, None)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    // COMPLETED FORMS

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn completed_forms(&self, ctx: &Context<'_>) -> Result<Vec<CompletedForm>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_completed_forms_by_user_id(&mut conn, user_id, None)
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

    // TESTING
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
    async fn login(&self, ctx: &Context<'_>, login_data: Login) -> Result<User, BigError> {
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

    async fn add_form(
        &self,
        ctx: &Context<'_>,
        form_input: FormInput,
    ) -> Result<FormOutput, BigError> {
        add_form(ctx, form_input).await
    }
    // update_logger() {}
    // delete_logger() {}

    // LOG STUFF
    async fn add_completed_form(
        &self,
        ctx: &Context<'_>,
        completed_form_input: CompletedFormInput,
    ) -> Result<CompletedFormOutput, BigError> {
        insert_completed_form(ctx, completed_form_input).await
    }

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
