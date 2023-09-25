use crate::apnea_forms::actions::get_forms_by_user::get_forms_by_user;
use actix_web::web;
use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    auth::actions::get_user_id_from_auth,
    graphql_schema::DbPool,
    utility::{
        errors::BigError,
        gql::{guards::LoggedInGuard, query_dto::QueryParams},
    },
};

use super::{
    dto::form_dto::{Form, FormDetails},
    forms_interface::FormRequest,
};

#[derive(Default)]
pub struct ApneaFormsQuery;

#[Object]
impl ApneaFormsQuery {
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn forms(
        &self,
        ctx: &Context<'_>,
        query_params: QueryParams,
    ) -> Result<Option<Vec<Form>>, BigError> {
        // TODO: Data loader and pagination
        let user_id = get_user_id_from_auth(ctx).await?;
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let forms = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_forms_by_user(&mut conn, user_id, query_params)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })??;

        Ok(forms)
    }
}

#[derive(Default)]
pub struct ApneaFormsMutation;

#[Object]
impl ApneaFormsMutation {
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn insert_form(
        &self,
        ctx: &Context<'_>,
        form_details: FormDetails,
        form_request: FormRequest,
    ) -> Result<Option<Form>, BigError> {
        let user_id = get_user_id_from_auth(ctx).await?;
        match form_request {
            FormRequest::V1(v1) => v1.insert_form(ctx, form_details, &user_id).await,
        }
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn modify_form(
        &self,
        ctx: &Context<'_>,
        previous_form_id: Uuid,
        form_details: FormDetails,
        form_request: FormRequest,
    ) -> Result<Option<Form>, BigError> {
        let user_id = get_user_id_from_auth(ctx).await?;
        match form_request {
            FormRequest::V1(v1) => {
                v1.modify_form(ctx, &previous_form_id, form_details, &user_id)
                    .await
            }
        }
    }
}
