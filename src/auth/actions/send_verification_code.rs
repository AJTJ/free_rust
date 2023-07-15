use crate::auth::dto::user_dto::{User, UserRetrievalData, UserUpdate};
use crate::auth::utility::auth_data::UniversalIdType;
use crate::diesel::ExpressionMethods;
use crate::graphql_schema::DbPool;
use crate::schema::users::{hashed_password, password_salt};
use crate::utility::errors::BigError;
use actix_web::web;
use argon2::{self, Config};
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use rand::Rng;
use uuid::Uuid;

use super::{get_user, get_user_id_from_auth};

pub async fn send_verification_code(
    ctx: &Context<'_>,
    unverified_user_id: &Uuid,
    unverified_email: String,
) -> Result<User, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    // 1. Use an email verification service here to send an email
    // https://sendgrid.com/pricing/
    // 2. I should limit how often a new verification code can be sent

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselUpdateError { source: e })?;
}
