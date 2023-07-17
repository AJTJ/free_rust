use crate::{
    graphql_schema::DbPool,
    utility::{
        errors::BigError,
        gql::guards::{DevelopmentGuard, LoggedInGuard},
    },
};
use actix_web::web;
use async_graphql::{Context, Object};
use diesel::RunQueryDsl;
use tracing::{debug_span, event, info, instrument, span, Level};

use super::{
    actions::{
        email_verification_code, get_user, get_user_id_from_auth, insert_unverified_user, login,
        logout, verify_email_code,
    },
    dto::{
        auth_dto::Login,
        user_dto::{User, UserInput, UserRetrievalData},
    },
};

#[derive(Default)]
pub struct AuthQuery;

#[derive(Default, Debug)]
pub struct AuthMutation;

#[Object]
impl AuthQuery {
    #[graphql(guard = "DevelopmentGuard::new()")]
    async fn all_users(&self, ctx: &Context<'_>) -> Result<Vec<User>, BigError> {
        let span = debug_span!("all_users");
        let all_users = span
            .in_scope(async || {
                // run some synchronous code inside the span...

                let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

                web::block(move || {
                    let mut conn = pool_ctx.get().unwrap();
                    event!(Level::DEBUG, "requested all users");
                    use crate::schema::users::dsl::*;
                    users.load::<User>(&mut conn)
                })
                .await
                .map_err(|e| BigError::ActixBlockingError { source: e })?
                .map_err(|e| BigError::DieselQueryError { source: e })
            })
            .await;

        all_users
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn user(&self, ctx: &Context<'_>, email: String) -> Result<User, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_user(&mut conn, UserRetrievalData::Email(email))
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

#[Object]
impl AuthMutation {
    // Must be UNGUARDED?
    async fn insert_user(
        &self,
        ctx: &Context<'_>,
        user_input: UserInput,
    ) -> Result<User, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let my_user_input = user_input.clone();
        let user = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            insert_unverified_user(&mut conn, &my_user_input)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselInsertError { source: e })?;

        login(ctx, user.email, user_input.password).await
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
    #[instrument(skip_all, name = "login_span", level = "debug")]
    async fn login(&self, ctx: &Context<'_>, login_data: Login) -> Result<User, BigError> {
        println!("HIT A");
        let thing = login(ctx, login_data.email, login_data.password).await;
        println!("HIT B");
        thing
    }

    // #[graphql(guard = "LoggedInGuard::new()")]
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, BigError> {
        let res = logout(ctx).await;
        res
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn email_verification_code(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<bool, BigError> {
        let user_id = get_user_id_from_auth(ctx).await?;
        email_verification_code(ctx, &user_id, email).await
    }

    #[graphql(guard = "LoggedInGuard::new()")]
    async fn verify_email_code(
        &self,
        ctx: &Context<'_>,
        email: String,
        email_code: String,
    ) -> Result<User, BigError> {
        let user_id = get_user_id_from_auth(ctx).await?;
        verify_email_code(ctx, &user_id, &email, &email_code).await
    }
}
