use crate::actions::{get_user_with_email, other_add_to_sesh};
use crate::auth_data::{SessionData, UniversalIdType};
use crate::errors::ErrorEnum;
use crate::graphql_schema::DbPool;
use crate::{actions::add_to_session::add_to_session, data::UserQueryData};
use actix_web::http::header::SET_COOKIE;
use actix_web::web;
use argon2::{self};
use async_graphql::Context;
use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use rand::{random, Rng};
use tracing::info;

pub async fn login(
    ctx: &Context<'_>,
    inc_email: String,
    password: String,
) -> Result<UserQueryData, ErrorEnum> {
    info!("login hit");
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
                    let id: UniversalIdType = Rng::gen::<UniversalIdType>(&mut rand::thread_rng());
                    let encoded_session_id = general_purpose::STANDARD.encode(id);

                    add_to_session(
                        ctx,
                        SessionData {
                            user_id: user.user_id,
                            expiry: Utc::now().naive_utc() + Duration::minutes(10080),
                        },
                        encoded_session_id.clone(),
                    )
                    .await;

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

                    info!("post add to session hit");
                    // TODO: This cookie needs an expiry
                    ctx.insert_http_header(SET_COOKIE, "free-rust-cookie");
                    ctx.append_http_header("set-cookie", encoded_session_id);
                    Ok(user)
                }
                false => Err(ErrorEnum::WrongPassword(password)),
            }
        }
        Err(e) => Err(ErrorEnum::UserNotFound(e)),
    };
    info!("the return user: {:?}", return_user);
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
