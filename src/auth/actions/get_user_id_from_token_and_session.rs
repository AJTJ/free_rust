use async_graphql::Context;
use uuid::Uuid;

use crate::{auth::utility::token_helpers::get_cookie_from_token, utility::errors::BigError};

pub async fn get_user_id_from_token_and_session(ctx: &Context<'_>) -> Result<Uuid, BigError> {
    // let cookie = get_cookie_from_token(ctx)?;

    // if let Some(session_id) = cookie.encoded_session_id {
    //     let user_session = get_user_session_data(ctx, session_id)
    //         .await
    //         .map_err(|e| BigError::RedisSessionError { source: e })?;
    //     Ok(user_session.user_id)
    // } else {
    //     Err(BigError::NoSessionIDOnToken)
    // }

    unimplemented!()
}
