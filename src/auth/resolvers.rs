use async_graphql::{types::connection::*, Context, Object};

use crate::errors::BigError;
use crate::guards::LoggedInGuard;

use super::{
    formV1::form::FormOutputV1,
    helpers::{AllFormsInput, AllFormsOutput},
};

#[derive(Default)]
pub struct Query;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Query {
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
}
