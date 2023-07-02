use super::{
    actions::{
        add_dive_session, get_dive_sessions_by_user, get_dives_by_user, update_dive,
        update_dive_session,
    },
    dto::{
        dive_dto::{Dive, DiveFilter, DiveInput, DiveUpdate},
        dive_session_dto::{DiveSession, DiveSessionFilter, DiveSessionInput, DiveSessionUpdate},
    },
};
use crate::{
    auth::actions::get_user_id_from_token_and_session,
    graphql_schema::DbPool,
    utility::{
        errors::{ActixBlockingSnafu, BigError},
        gql::{
            graphql_query::gql_query,
            guards::{DevelopmentGuard, LoggedInGuard},
            query_dto::QueryParams,
        },
    },
};
use actix_web::web;
use async_graphql::{types::connection::*, Context, Object};
use diesel::RunQueryDsl;
use uuid::Uuid;

#[derive(Default)]
pub struct Query;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Query {
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
}

#[Object]
impl Mutation {
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
}
