use async_graphql::{types::connection::*, Context, EmptySubscription, Object, Schema};

use crate::errors::BigError;
use crate::guards::{DevelopmentGuard, LoggedInGuard};

use super::form_1::{FormInputV1, FormOutputV1};

#[derive(Default)]
pub struct Query;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Query {
    // they simply get all the forms they want
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn get_forms_new(&self, _ctx: &Context<'_>) -> Result<Vec<FormOutputV1>, BigError> {
        unimplemented!()
    }

    // they simply get all the forms they want
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn get_completed_forms_new(
        &self,
        _ctx: &Context<'_>,
    ) -> Result<Vec<FormOutputV1>, BigError> {
        unimplemented!()
    }
}

#[Object]
impl Mutation {
    // add one get one
    // MODIFY/ARCHIVE WOULD BE VERY SIMILAR
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn add_new_form_new(
        &self,
        _ctx: &Context<'_>,
        forms_input: FormInputV1,
    ) -> Result<FormOutputV1, BigError> {
        // from the particular form, add it and output it
        unimplemented!()
    }

    // add a new one
    // MODIFY/ARCHIVE WOULD BE VERY SIMILAR
    #[graphql(guard = "LoggedInGuard::new()")]
    async fn add_completed_form_new(
        &self,
        _ctx: &Context<'_>,
        forms_input: FormInputV1,
    ) -> Result<FormOutputV1, BigError> {
        // from the particular form, add it and output it

        unimplemented!()
    }
}
