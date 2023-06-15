use crate::{cookie_helpers::get_cookie_from_token, errors::SessionCookieErrors};
// use actix_session::Session;
use super::get_user_session_data;
use async_graphql::Context;
use uuid::Uuid;

pub async fn get_user_id_from_cookie_session(
    ctx: &Context<'_>,
) -> Result<Uuid, SessionCookieErrors> {
    let cookie = get_cookie_from_token(ctx).map_err(|e| SessionCookieErrors::CookieError(e))?;

    let user_session = get_user_session_data(ctx, cookie.encoded_session_id)
        .await
        .map_err(|e| SessionCookieErrors::SessionError(e))?;
    Ok(user_session.user_id)
}
