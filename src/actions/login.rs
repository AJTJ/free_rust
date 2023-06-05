use crate::actions::get_user_with_email;
use crate::auth_data::{SessionData, UniversalIdType};
use crate::cookie_helpers::create_cookie;
use crate::errors::ErrorEnum;
use crate::graphql_schema::DbPool;
use crate::helpers::get_encoded_id;
use crate::{actions::add_to_user_session::add_to_user_session, dto::user_auth_dto::UserQueryData};
use actix_web::http::header::SET_COOKIE;
use actix_web::web;
use argon2::{self};
use async_graphql::Context;

use chrono::{Duration, Utc};
use rand::Rng;

pub async fn login(
    ctx: &Context<'_>,
    inc_email: String,
    password: String,
) -> Result<UserQueryData, ErrorEnum> {
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

                    Ok(user)
                }
                false => Err(ErrorEnum::WrongPassword(password)),
            }
        }
        Err(e) => Err(ErrorEnum::UserNotFound(e)),
    };
    return_user
}

// pub async fn login(
//     ctx: &Context<'_>,
//     inc_email: String,
//     password: String,
//     encoded_session_id: String,
// ) -> Result<UserQueryData, ErrorEnum> {
//     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
//     let maybe_user = web::block(move || {
//         let mut conn = pool_ctx.get().unwrap();
//         get_user_with_email(&mut conn, inc_email)
//     })
//     .await
//     .expect("maybe_user error in login");

//     match maybe_user {
//         Ok(user) => {
//             // check if pw matches
//             let passwords_match =
//                 argon2::verify_encoded(&user.hashed_password, password.as_bytes()).unwrap();

//             match passwords_match {
//                 true => {
//                     // Normally I would add to session here, but it is being passed up.

//                     // TODO: This cookie needs an expiry
//                     ctx.insert_http_header(SET_COOKIE, "free-rust-cookie");
//                     ctx.append_http_header("set-cookie", encoded_session_id);
//                     Ok(user)
//                 }
//                 false => Err(ErrorEnum::WrongPassword(password)),
//             }
//         }
//         Err(e) => Err(ErrorEnum::UserNotFound(e)),
//     }
// }

// if random() {
//     info!("positive");
//     add_to_session(
//         ctx,
//         SessionData {
//             user_id: user.user_id,
//             expiry: Utc::now().naive_utc() + Duration::minutes(10080),
//         },
//         encoded_session_id.clone(),
//     );
// } else {
//     info!("negative");
//     other_add_to_sesh(
//         ctx,
//         SessionData {
//             user_id: user.user_id,
//             expiry: Utc::now().naive_utc() + Duration::minutes(10080),
//         },
//         encoded_session_id.clone(),
//     );
// }
