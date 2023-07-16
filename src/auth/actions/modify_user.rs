use crate::auth::dto::user_dto::{User, UserRetrievalData, UserUpdate};
use crate::auth::utility::auth_data::UniversalIdType;
use crate::diesel::ExpressionMethods;
use crate::graphql_schema::DbPool;
use crate::schema::users::{hashed_password, password_salt};
use crate::utility::errors::BigError;
use actix_web::web;
use argon2::{self, Config};
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use rand::Rng;
use uuid::Uuid;

use super::get_user_id_from_auth;

pub async fn modify_user(
    ctx: &Context<'_>,
    new_password: Option<String>,
    input_user_id: Option<&Uuid>,
    user_mod_data: UserUpdate,
) -> Result<User, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    // NOTE: get user_id from cookie/session if it isn't included, since this is used as part of the login process
    let input_user_id = match input_user_id {
        Some(u) => *u,
        None => get_user_id_from_auth(ctx).await?,
    };

    let my_user_mod_data = user_mod_data.clone();

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        use crate::schema::users::dsl::{id as user_id, updated_at, users};
        let update = diesel::update(users)
            .filter(user_id.eq(&input_user_id))
            .into_boxed();
        if let Some(pw) = new_password {
            let salt_gen: UniversalIdType = rand::thread_rng().gen::<UniversalIdType>();
            let hashed_pw =
                argon2::hash_encoded(pw.as_bytes(), &salt_gen, &Config::default()).unwrap();

            update
                .set((
                    &my_user_mod_data,
                    updated_at.eq(Utc::now()),
                    hashed_password.eq(hashed_pw),
                    password_salt.eq(salt_gen.to_vec()),
                ))
                .get_result::<User>(&mut conn)
        } else {
            update
                .set((&my_user_mod_data, updated_at.eq(Utc::now())))
                .get_result::<User>(&mut conn)
        }
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselUpdateError { source: e })
}
