use crate::apnea_forms::actions::get_forms::get_forms;
use actix_web::web;
use async_graphql::{types::connection::*, Context, Object};
use tracing::info;
use uuid::Uuid;

use crate::{
    auth::actions::get_user_id_from_auth,
    graphql_schema::DbPool,
    utility::{
        errors::BigError,
        gql::{graphql_query::gql_query, guards::LoggedInGuard, query_dto::QueryParams},
    },
};

use super::{
    actions::get_reports::get_reports,
    dto::{
        form_dto::{Form, FormDetails},
        report_dto::{Report, ReportDetails, ReportsRetrievalData},
    },
    form_v1::form::{self, FormResponseV1},
    helpers::{FormRequest, FormResponse},
};

#[derive(Default)]
pub struct ApneaFormsQuery;

#[derive(Default)]
pub struct ApneaFormsMutation;

#[Object]
impl ApneaFormsQuery {
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn forms(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Vec<Form>, BigError> {
        // TODO: Data loader and pagination
        let user_id = get_user_id_from_auth(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let forms = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_forms(&mut conn, user_id, query_params)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })??;

        Ok(forms)
    }

    // they simply get all the forms they want
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn reports(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Connection<String, Report>, BigError> {
        // TODO: Add dataloader?

        let user_id = get_user_id_from_auth(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let my_closure = move |query_params: QueryParams| {
            let query_params = query_params.clone();
            let pool_ctx = pool_ctx.clone();
            async move {
                web::block(move || {
                    let mut conn = pool_ctx.get().unwrap();
                    get_reports(
                        &mut conn,
                        ReportsRetrievalData::UserId(user_id),
                        query_params,
                    )
                })
                .await
                .map_err(|e| BigError::ActixBlockingError { source: e })?
            }
        };

        let query_response = gql_query(query_params, &my_closure).await;
        query_response.map_err(|e| BigError::AsyncQueryError { error: e })
    }
}

#[Object]
impl ApneaFormsMutation {
    // add one get one
    // MODIFY/ARCHIVE WOULD BE VERY SIMILAR
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn insert_form(
        &self,
        ctx: &Context<'_>,
        form_details_input: FormDetails,
        form_request: FormRequest,
    ) -> Result<Form, BigError> {
        match form_request {
            FormRequest::V1(v1) => {
                FormResponseV1::from(v1)
                    .insert_form(ctx, form_details_input)
                    .await
            }
        }
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn modify_form(
        &self,
        ctx: &Context<'_>,
        previous_form_id: Uuid,
        form_request: FormRequest,
    ) -> Result<Form, BigError> {
        match form_request {
            FormRequest::V1(v1) => {
                FormResponseV1::from(v1)
                    .modify_form(ctx, previous_form_id)
                    .await
            }
        }
    }

    // add a new one
    // MODIFY/ARCHIVE WOULD BE VERY SIMILAR
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn insert_report(
        &self,
        ctx: &Context<'_>,
        session_id: Uuid,
        report_details_input: ReportDetails,
        report_input: FormRequest,
    ) -> Result<Report, BigError> {
        // info!("report_input: {report_input:?}");
        match report_input {
            FormRequest::V1(v1) => {
                FormResponseV1::from(v1)
                    .insert_report(ctx, &session_id, report_details_input)
                    .await
            }
        }
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn modify_report(
        &self,
        ctx: &Context<'_>,
        previous_report_id: Uuid,
        forms_input: FormRequest,
    ) -> Result<Report, BigError> {
        match forms_input {
            FormRequest::V1(v1) => {
                FormResponseV1::from(v1)
                    .modify_report(ctx, previous_report_id)
                    .await
            }
        }
    }
}
