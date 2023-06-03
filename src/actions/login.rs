use std::error::Error;
use std::fmt;

use crate::auth_data::SessionData;
use crate::diesel::ExpressionMethods;
use crate::errors::{MyError, WrongPassword};
use crate::graphql_schema::DbPool;
use crate::{actions::add_to_session::add_to_session, data::UserQueryData};

use actix_web::web;
use anyhow::anyhow;
use async_graphql::Context;
use chrono::{Duration, Utc};
use diesel::{QueryDsl, RunQueryDsl};

pub async fn login(
    ctx: &Context<'_>,
    inc_email: String,
    password: String,
) -> diesel::QueryResult<UserQueryData> {
    use crate::schema::users::dsl::*;

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let maybe_user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let maybe_user = users
            .filter(email.eq(&inc_email))
            .first::<UserQueryData>(&mut conn);

        maybe_user
    })
    .await
    .expect("maybe_user error in login");

    match maybe_user {
        Ok(user) => {
            // check if pw matches
            let passwords_match = argon2::verify_encoded(&user.hashed_password, password).unwrap();

            match passwords_match {
                true => {
                    add_to_session(
                        ctx,
                        SessionData {
                            user_id: user.user_id,
                            expiry: Utc::now().naive_utc() + Duration::minutes(10080),
                        },
                    );
                    Ok(user)
                }
                false => (),
            }
        }
        Err(e) => Err(e),
    }
}
