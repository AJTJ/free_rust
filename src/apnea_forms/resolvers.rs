use actix_web::web;
use async_graphql::{types::connection::*, Context, Object};
use uuid::Uuid;

use crate::{
    auth::actions::get_user_id_from_token_and_session,
    graphql_schema::DbPool,
    utility::{
        errors::BigError,
        gql::{graphql_query::gql_query, guards::LoggedInGuard, query_dto::QueryParams},
    },
};

use super::{
    actions::get_reports::get_reports,
    dto::{form_dto::FormDetailsInput, report_dto::ReportDetailsInput},
    formV1::form::FormOutputV1,
    helpers::{FormInput, FormOutput},
};

#[derive(Default)]
pub struct Query;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Query {
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn forms(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Vec<FormOutput>, BigError> {
        // TODO: Query the database and get a `Vec<AllFormsOutput>`
        // Does it need to be validated in the process?
        // TODO: Add pagination and dataloader
        unimplemented!()
    }

    // they simply get all the forms they want
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn reports(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Connection<String, FormOutput>, BigError> {
        // TODO: Query the database and get a `Vec<AllFormsOutput>`
        // TODO: Add dataloader?

        let user_id = get_user_id_from_token_and_session(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let my_closure = move |query_params: QueryParams| {
            let query_params = query_params.clone();
            let pool_ctx = pool_ctx.clone();
            async move {
                web::block(move || {
                    let mut conn = pool_ctx.get().unwrap();
                    get_reports(&mut conn, user_id, query_params)
                })
                .await
                .map_err(|e| BigError::ActixBlockingError { source: e })?
            }
        };

        let query_response = gql_query(query_params, &my_closure).await;
        query_response.map_err(|e| BigError::AsyncQueryError { error: e })
    }

    // #[graphql(guard = "LoggedInGuard::new()")]
    // async fn completed_forms(
    //     &self,
    //     ctx: &Context<'_>,
    //     query_params: QueryParams,
    // ) -> Result<Connection<String, FormStructureOutput>, BigError> {
    //     let user_id = get_user_id_from_token_and_session(ctx).await?;
    //     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    //     let my_closure = move |query_params: QueryParams| {
    //         let query_params = query_params.clone();
    //         let pool_ctx = pool_ctx.clone();
    //         async move {
    //             web::block(move || {
    //                 let mut conn = pool_ctx.get().unwrap();
    //                 get_completed_forms_by_user_id(&mut conn, user_id, query_params)
    //             })
    //             .await
    //             .map_err(|e| BigError::ActixBlockingError { source: e })?
    //         }
    //     };

    //     let query_response = gql_query(query_params, &my_closure).await;
    //     query_response.map_err(|e| BigError::AsyncQueryError { error: e })
    // }
}

#[Object]
impl Mutation {
    // add one get one
    // MODIFY/ARCHIVE WOULD BE VERY SIMILAR
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn insert_form(
        &self,
        ctx: &Context<'_>,
        form_details_input: FormDetailsInput,
        form_input: FormInput,
    ) -> Result<FormOutput, BigError> {
        match form_input {
            FormInput::V1(v1) => {
                FormOutputV1::from(v1)
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
        form_input: FormInput,
    ) -> Result<FormOutput, BigError> {
        match form_input {
            FormInput::V1(v1) => {
                FormOutputV1::from(v1)
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
        report_details_input: ReportDetailsInput,
        report_input: FormInput,
    ) -> Result<FormOutput, BigError> {
        match report_input {
            FormInput::V1(v1) => {
                FormOutputV1::from(v1)
                    .insert_report(ctx, report_details_input)
                    .await
            }
        }
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn modify_report(
        &self,
        ctx: &Context<'_>,
        previous_report_id: Uuid,
        forms_input: FormInput,
    ) -> Result<FormOutput, BigError> {
        match forms_input {
            FormInput::V1(v1) => {
                FormOutputV1::from(v1)
                    .modify_form(ctx, previous_report_id)
                    .await
            }
        }
    }
}

// #[graphql(guard = "LoggedInGuard::new()")]
// async fn form_structures(&self, _ctx: &Context<'_>) -> FormStructureOutput {
//     get_form_structures()
// }

// #[graphql(guard = "LoggedInGuard::new()")]
// async fn forms(&self, ctx: &Context<'_>) -> Result<Vec<FormOutput>, BigError> {
//     let user_id = get_user_id_from_token_and_session(ctx).await?;
//     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
//     web::block(move || {
//         let mut conn = pool_ctx.get().unwrap();
//         get_forms_by_user_id(&mut conn, user_id, None)
//     })
//     .await
//     .map_err(|e| BigError::ActixBlockingError { source: e })?
// }

// #[graphql(guard = "LoggedInGuard::new()")]
// async fn form_fields(
//     &self,
//     ctx: &Context<'_>,
//     logger_id: Uuid,
// ) -> Result<Vec<FormField>, BigError> {
//     let user_id = get_user_id_from_token_and_session(ctx).await?;
//     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
//     web::block(move || {
//         let mut conn = pool_ctx.get().unwrap();
//         get_form_fields_by_form(&mut conn, &logger_id, &user_id, None)
//     })
//     .await
//     .map_err(|e| BigError::ActixBlockingError { source: e })?
//     .map_err(|e| BigError::DieselQueryError { source: e })
// }

// // COMPLETED FORMS
// #[graphql(guard = "LoggedInGuard::new()")]
// async fn completed_forms(
//     &self,
//     ctx: &Context<'_>,
//     query_params: QueryParams,
// ) -> Result<Connection<String, FormStructureOutput>, BigError> {
//     let user_id = get_user_id_from_token_and_session(ctx).await?;
//     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

//     let my_closure = move |query_params: QueryParams| {
//         let query_params = query_params.clone();
//         let pool_ctx = pool_ctx.clone();
//         async move {
//             web::block(move || {
//                 let mut conn = pool_ctx.get().unwrap();
//                 get_completed_forms_by_user_id(&mut conn, user_id, query_params)
//             })
//             .await
//             .map_err(|e| BigError::ActixBlockingError { source: e })?
//         }
//     };

//     let query_response = gql_query(query_params, &my_closure).await;
//     query_response.map_err(|e| BigError::AsyncQueryError { error: e })
// }

// async fn add_form(
//     &self,
//     ctx: &Context<'_>,
//     form_input: FormInput,
// ) -> Result<FormStructureOutput, BigError> {
//     add_form(ctx, form_input).await
// }

// // LOG STUFF
// async fn add_completed_form(
//     &self,
//     ctx: &Context<'_>,
//     completed_form_input: CompletedFormInput,
// ) -> Result<FormStructureOutput, BigError> {
//     insert_completed_form(ctx, completed_form_input).await
// }
