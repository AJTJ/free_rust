use crate::actions::get_user_with_email;
use crate::auth_data::{SessionData, COOKIE_NAME};
use crate::errors::ErrorEnum;
use crate::graphql_schema::DbPool;
use crate::{actions::add_to_session::add_to_session, data::UserQueryData};
use actix_web::web;
use async_graphql::Context;
use chrono::{Duration, Utc};
use tracing::log::info;

pub fn logout(ctx: &Context<'_>) -> bool {
    match ctx.http_header_contains(COOKIE_NAME) {
        true => {
            info!("it contains cookie name");
            // TODO: decode the session_id
            // TODO: remove the session_id from the session
            // TODO: delete the cookie
            true
        }
        false => {
            info!("it doesn't contain cookie name");
            // TODO: insert meaningful error
            false
        }
    }
}
