use crate::actions::add_dive;
use crate::actions::add_dive_session;
use crate::actions::add_form;
use crate::actions::get_completed_forms_by_user_id;
use crate::actions::get_dive_sessions_by_user;
use crate::actions::get_dives_by_user;
use crate::actions::get_form_fields_by_form;
use crate::actions::get_form_structures;
use crate::actions::get_forms_by_user_id;
use crate::actions::get_user_id_from_token_and_session;
use crate::actions::get_user_with_email;
use crate::actions::insert_completed_form;
use crate::actions::insert_user;
use crate::actions::login;
use crate::actions::logout;
use crate::actions::update_dive;
use crate::actions::update_dive_session;
use crate::dive_forms::form_1::FormInputV1;
use crate::dive_forms::form_1::FormOutputV1;
use crate::dive_forms::form_helper::FormStructureOutput;
use crate::dive_forms::form_trait::FormInputNew;
use crate::dive_forms::form_trait::FormTrait;
use crate::dive_forms::helpers::AllFormVersions;
use crate::dive_forms::helpers::AllFormVersionsInput;
use crate::dive_forms::helpers::FormVersion;
use crate::dive_forms::resolvers::{Mutation as FormMutation, Query as FormQuery};
use crate::dto::auth_dto::Login;
use crate::dto::completed_form_dto::CompletedFormInput;
use crate::dto::dive_dto::Dive;
use crate::dto::dive_dto::DiveFilter;
use crate::dto::dive_dto::DiveInput;
use crate::dto::dive_dto::DiveUpdate;
use crate::dto::dive_session_dto::DiveSession;
use crate::dto::dive_session_dto::DiveSessionFilter;
use crate::dto::dive_session_dto::DiveSessionInput;
use crate::dto::dive_session_dto::DiveSessionUpdate;
use crate::dto::form_dto::FormInput;
use crate::dto::form_dto::FormOutput;
use crate::dto::form_field_dto::FormField;
use crate::dto::query_dto::QueryParams;
use crate::dto::user_dto::{User, UserInput};
use crate::errors::{ActixBlockingSnafu, BigError};
use crate::graphql_query::gql_query;
use crate::guards::{DevelopmentGuard, LoggedInGuard};
use actix_web::web;
use async_graphql::MergedObject;
use async_graphql::{types::connection::*, Context, EmptySubscription, Object, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use rand::prelude::*;
use snafu::ResultExt;
use uuid::Uuid;

pub type DiveQLSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(FormQuery, RootQuery);
#[derive(Default)]
pub struct RootQuery;

#[derive(MergedObject, Default)]
pub struct Mutation(RootMutation, FormMutation);
#[derive(Default)]
pub struct RootMutation;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[Object]
impl RootQuery {
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
        .map_err(|e| BigError::ActixBlockingError { source: e })?
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
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        dive_session_filter: Option<DiveSessionFilter>,
        query_params: QueryParams,
    ) -> Result<Connection<String, DiveSession>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user_id = get_user_id_from_token_and_session(ctx).await?;

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
                .context(ActixBlockingSnafu)?
            }
        };

        let query_response = gql_query(query_params, &my_closure).await;
        query_response.map_err(|e| BigError::AsyncQueryError { error: e })
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
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    // FORMS

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn form_structures(&self, _ctx: &Context<'_>) -> FormStructureOutput {
        get_form_structures()
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn forms(&self, ctx: &Context<'_>) -> Result<Vec<FormOutput>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_forms_by_user_id(&mut conn, user_id, None)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
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
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }

    // COMPLETED FORMS
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn completed_forms(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Connection<String, FormStructureOutput>, BigError> {
        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let my_closure = move |query_params: QueryParams| {
            let query_params = query_params.clone();
            let pool_ctx = pool_ctx.clone();
            async move {
                web::block(move || {
                    let mut conn = pool_ctx.get().unwrap();
                    get_completed_forms_by_user_id(&mut conn, user_id, query_params)
                })
                .await
                .map_err(|e| BigError::ActixBlockingError { source: e })?
            }
        };

        let query_response = gql_query(query_params, &my_closure).await;
        query_response.map_err(|e| BigError::AsyncQueryError { error: e })
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn guarded_route(&self, _ctx: &Context<'_>) -> f64 {
        // Ok("Made it".to_string())
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        y
    }
}

#[Object]
impl RootMutation {
    // Must be UNGUARDED?
    async fn insert_user(&self, ctx: &Context<'_>, user_data: UserInput) -> Result<User, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            insert_user(&mut conn, user_data)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
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
        .map_err(|e| BigError::ActixBlockingError { source: e })?
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
        .map_err(|e| BigError::ActixBlockingError { source: e })?
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
    ) -> Result<FormStructureOutput, BigError> {
        add_form(ctx, form_input).await
    }
    // update_logger() {}
    // delete_logger() {}

    // LOG STUFF
    async fn add_completed_form(
        &self,
        ctx: &Context<'_>,
        completed_form_input: CompletedFormInput,
    ) -> Result<FormStructureOutput, BigError> {
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
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselDeleteError { source: e })
    }

    // async fn add_test_form_v_1_0_0(&self, input: TestFormInput<FormInput1_0_0>) -> i32 {
    //     42
    // }
    // async fn add_test_form_v_2_0_0(&self, input: TestFormInput<FormInput2_0_0>) -> i32 {
    //     42
    // }
}

// #[async_trait]
// pub trait DealsWithForms<T: FormTrait + InputType> {
//     async fn add_test_form(&self, input_form: TestFormInput<T>) -> i32
//     where
//         T: 'async_trait;
// }

// #[async_trait]
// impl<T: FormTrait + InputType + Send> DealsWithForms<T> for Mutation {
//     async fn add_test_form(&self, input_form: TestFormInput<T>) -> i32
//     where
//         T: 'async_trait,
//     {
//         42
//     }
// }
