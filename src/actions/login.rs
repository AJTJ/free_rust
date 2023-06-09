use crate::actions::add_to_user_session::add_to_user_session;
use crate::actions::get_user_with_email;
use crate::auth_data::{SessionData, UniversalIdType};
use crate::cookie_helpers::create_cookie;
use crate::dto::user_auth_dto::{UserModificationData, UserQueryDataOutput};
use crate::errors::ErrorEnum;
use crate::graphql_schema::DbPool;
use crate::helpers::get_encoded_id;
use actix_web::http::header::SET_COOKIE;
use actix_web::web;
use argon2::{self};
use async_graphql::Context;

use chrono::{Duration, Utc};
use rand::Rng;

use super::update_user;

pub async fn login(
    ctx: &Context<'_>,
    inc_email: String,
    password: String,
) -> Result<UserQueryDataOutput, ErrorEnum> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let maybe_user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_user_with_email(&mut conn, inc_email)
    })
    .await
    .expect("maybe_user error in login");

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
                            user_id: user.user_id,
                            expiry: Utc::now().naive_utc() + Duration::minutes(10080),
                        },
                        encoded_session_id.clone(),
                    )
                    .await;

                    let cookie = create_cookie(encoded_session_id);
                    ctx.insert_http_header(SET_COOKIE, cookie.to_string());

                    let updated_user = UserModificationData {
                        last_login: Some(Utc::now().naive_utc()),
                        username: None,
                        email: None,
                        is_active: None,
                    };

                    let updated_user =
                        update_user(ctx, None, Some(user.user_id), updated_user).await;

                    let user_out: UserQueryDataOutput = updated_user.into();

                    Ok(user_out)
                }
                false => Err(ErrorEnum::WrongPassword(password)),
            }
        }
        Err(e) => Err(ErrorEnum::UserNotFound(e)),
    };
    return_user
}
