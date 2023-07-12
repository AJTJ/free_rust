use super::{
    actions::{
        archive_dive, archive_session, get_apnea_sessions_paginated, insert_apnea_session,
        insert_dive,
    },
    dive_loader_by_user::DiveLoaderByUser,
    dto::{
        apnea_session_dto::{ApneaSession, ApneaSessionInput, ApneaSessionRetrievalData},
        dive_dto::{Dive, DiveInput, DiveRetrievalData},
    },
};
use crate::{apnea_forms::dto::report_dto::ReportDetails, diesel::RunQueryDsl};
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
use async_graphql::{dataloader::DataLoader, types::connection::*, Context, Object};
use snafu::ResultExt;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct ApneaSessionsQuery;

#[derive(Default)]
pub struct ApneaSessionsMutation;

#[Object]
impl ApneaSessionsQuery {
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn apnea_sessions(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Connection<String, ApneaSession>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let user_id = get_user_id_from_auth(ctx).await?;
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
        _query_params: Option<QueryParams>,
    ) -> Result<Vec<Dive>, Arc<BigError>> {
        let user_id = get_user_id_from_auth(ctx).await?;

        let dives_map = ctx
            .data_unchecked::<DataLoader<DiveLoaderByUser>>()
            .load_many([DiveRetrievalData::User(user_id)])
            .await?;

        let dives = dives_map.into_iter().map(|(_, v)| v).collect::<Vec<Dive>>();

        Ok(dives)
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
        report_details: Option<ReportDetails>,
    ) -> Result<ApneaSession, BigError> {
        let user_id = get_user_id_from_auth(ctx).await?;
        insert_apnea_session(ctx, apnea_session_input, report_details, &user_id).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn modify_apnea_session(
        &self,
        ctx: &Context<'_>,
        archived_session_id: Uuid,
        apnea_session_input: ApneaSessionInput,
        report_details: Option<ReportDetails>,
    ) -> Result<ApneaSession, BigError> {
        let user_id = get_user_id_from_auth(ctx).await?;
        archive_session(ctx, &archived_session_id, &user_id).await;
        insert_apnea_session(ctx, apnea_session_input, report_details, &user_id).await
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
    async fn modify_dive(
        &self,
        ctx: &Context<'_>,
        archived_dive_id: Uuid,
        apnea_session_id: Uuid,
        dive_input: DiveInput,
    ) -> Result<Dive, BigError> {
        let user_id = get_user_id_from_auth(ctx).await?;
        archive_dive(ctx, &archived_dive_id, &user_id).await?;
        insert_dive(ctx, apnea_session_id, dive_input).await
    }
}
