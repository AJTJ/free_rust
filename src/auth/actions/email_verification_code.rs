use crate::auth::actions::modify_user;
use crate::utility::errors::BigError;
use crate::{auth::dto::user_dto::UserUpdate, graphql_schema::DbPool};
use async_graphql::Context;
use chrono::{Days, Utc};
use rand::distributions::{Alphanumeric, DistString};
use tracing::info;
use uuid::Uuid;

pub async fn email_verification_code(
    ctx: &Context<'_>,
    unverified_user_id: &Uuid,
    unverified_email: String,
) -> Result<bool, BigError> {
    // check sendgrid.com

    let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

    let add_code_update = UserUpdate {
        last_login: None,
        username: None,
        email: None,
        is_active: None,
        is_email_verified: None,
        verification_code: Some(random_string.clone()),
        verification_code_expiry: Some(Utc::now().checked_add_days(Days::new(1)).unwrap()),
    };

    info!("random string: {random_string}");

    modify_user(ctx, None, Some(unverified_user_id), add_code_update).await?;

    fn send_email() -> bool {
        true
    }

    Ok(send_email())
}
