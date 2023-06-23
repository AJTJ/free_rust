use crate::actions::get_user_with_email;
use crate::actions::insert_into_user_session::add_to_user_session;
use crate::auth_data::{SessionData, UniversalIdType};
use crate::dto::user_dto::{User, UserUpdate};
use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::helpers::encoding_helpers::get_encoded_id;
use crate::helpers::token_helpers::{create_cookie, CUSTOM_HEADER};
use actix_web::http::header::{AUTHORIZATION, SET_COOKIE};
use actix_web::web;
use argon2::{self};
use async_graphql::Context;

use chrono::{Duration, Utc};
use rand::Rng;
use tracing::info;

use super::update_user;

pub async fn login(
    ctx: &Context<'_>,
    inc_email: String,
    password: String,
) -> Result<User, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let maybe_user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_user_with_email(&mut conn, inc_email)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?;

    let return_user = match maybe_user {
        Ok(user) => {
            // check if pw matches
            let passwords_match =
                argon2::verify_encoded(&user.hashed_password, password.as_bytes()).unwrap();

            match passwords_match {
                true => {
                    let session_id: UniversalIdType =
                        Rng::gen::<UniversalIdType>(&mut rand::thread_rng());
                    let encoded_session_id = get_encoded_id(session_id);

                    add_to_user_session(
                        ctx,
                        SessionData {
                            user_id: user.id,
                            expiry: Utc::now().naive_utc() + Duration::minutes(10080),
                        },
                        encoded_session_id.clone(),
                    )
                    .await;

                    let cookie = create_cookie(encoded_session_id);

                    ctx.insert_http_header(SET_COOKIE, cookie.to_string());
                    ctx.insert_http_header(AUTHORIZATION, cookie.to_string());

                    let updated_user = UserUpdate {
                        last_login: Some(Utc::now().naive_utc()),
                        username: None,
                        email: None,
                        is_active: None,
                    };

                    let updated_user = update_user(ctx, None, Some(user.id), updated_user).await?;

                    Ok(updated_user)
                }
                false => Err(BigError::WrongPassword),
            }
        }
        Err(e) => Err(BigError::DieselUserNotFound { source: e }),
    };

    return_user
}
