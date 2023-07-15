use crate::auth::dto::user_dto::{User, UserRetrievalData, UserUpdate};
use crate::auth::utility::auth_data::{SessionData, UniversalIdType};
use crate::auth::utility::encoding_helpers::get_encoded_id;
use crate::auth::utility::token_helpers::create_cookie;
use crate::graphql_schema::DbPool;
use crate::utility::errors::BigError;
use actix_web::http::header::{AUTHORIZATION, SET_COOKIE};
use actix_web::web;
use argon2::{self};
use async_graphql::Context;

use chrono::{Duration, Utc};
use rand::Rng;

use super::{get_user, insert_into_user_session, modify_user};

pub async fn login(
    ctx: &Context<'_>,
    inc_email: String,
    password: String,
) -> Result<User, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let maybe_user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_user(&mut conn, UserRetrievalData::Email(inc_email))
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

                    insert_into_user_session(
                        ctx,
                        SessionData {
                            user_id: user.id,
                            expiry: Utc::now() + Duration::minutes(10080),
                        },
                        encoded_session_id.clone(),
                    )
                    .await?;

                    let cookie = create_cookie(encoded_session_id);

                    ctx.insert_http_header(SET_COOKIE, cookie.to_string());
                    ctx.insert_http_header(AUTHORIZATION, cookie.to_string());

                    let updated_user = UserUpdate {
                        last_login: Some(Utc::now()),
                        username: None,
                        email: None,
                        is_active: None,
                        is_email_verified: None,
                    };

                    let updated_user = modify_user(ctx, None, Some(user.id), updated_user).await?;

                    Ok(updated_user)
                }
                false => Err(BigError::WrongPassword),
            }
        }
        Err(e) => Err(BigError::DieselUserNotFound { source: e }),
    };

    return_user
}
