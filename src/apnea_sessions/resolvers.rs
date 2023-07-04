use super::{
    actions::{
        get_apnea_sessions, get_dives, insert_apnea_session, insert_dive, update_apnea_session,
        update_dive,
    },
    dto::{
        apnea_session_dto::{
            ApneaSession, ApneaSessionFilter, ApneaSessionInput, ApneaSessionUpdate,
            ApnesSessionRetrievalData,
        },
        dive_dto::{Dive, DiveFilter, DiveInput, DiveRetrievalData, DiveUpdate},
    },
};
use crate::{apnea_forms::dto::report_dto::ReportDetailsInput, diesel::RunQueryDsl};
use crate::{
    auth::actions::get_user_id_from_auth,
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
use snafu::ResultExt;
use tracing::info;
use uuid::Uuid;

#[derive(Default)]
pub struct ApneaSessionsQuery;

#[derive(Default)]
pub struct ApneaSessionsMutation;

#[Object]
impl ApneaSessionsQuery {
    // #[graphql(guard = "LoggedInGuard::new()")]
    async fn apnea_sessions(
        &self,
        ctx: &Context<'_>,
        apnea_session_filter: Option<ApneaSessionFilter>,
        query_params: QueryParams,
    ) -> Result<Connection<String, ApneaSession>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user_id = get_user_id_from_auth(ctx).await?;
        let my_closure = move |query_params: QueryParams| {
            let query_params = query_params.clone();
            let apnea_session_filter = apnea_session_filter.clone();
            let pool_ctx = pool_ctx.clone();
            async move {
                web::block(move || {
                    let mut conn = pool_ctx.get().unwrap();
                    get_apnea_sessions(
                        &mut conn,
                        ApnesSessionRetrievalData::User(user_id),
                        apnea_session_filter,
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
        let user_id = get_user_id_from_auth(ctx).await?;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dives(
                &mut conn,
                DiveRetrievalData::User(user_id),
                dive_input,
                db_query_dto,
            )
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

#[Object]
impl ApneaSessionsMutation {
    // DIVE SESSION
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn insert_apnea_session(
        &self,
        ctx: &Context<'_>,
        apnea_session_input: ApneaSessionInput,
        report_details: Option<ReportDetailsInput>,
    ) -> Result<ApneaSession, BigError> {
        let session = insert_apnea_session(ctx, apnea_session_input, report_details).await;

        info!("session: {session:?}");

        session
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn update_apnea_session(
        &self,
        ctx: &Context<'_>,
        apnea_session_update: ApneaSessionUpdate,
    ) -> Result<ApneaSession, BigError> {
        update_apnea_session(ctx, apnea_session_update).await
    }

    // for testing
    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn delete_all_apnea_sessions(&self, ctx: &Context<'_>) -> Result<usize, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            use crate::schema::apnea_sessions::dsl::apnea_sessions;
            diesel::delete(apnea_sessions).execute(&mut conn)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselDeleteError { source: e })
    }
    // DIVES
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn insert_dive(
        &self,
        ctx: &Context<'_>,
        apnea_session_id: Uuid,
        dive_input: DiveInput,
    ) -> Result<Dive, BigError> {
        insert_dive(ctx, apnea_session_id, dive_input).await
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
