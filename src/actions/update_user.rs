use crate::actions::{get_user_session_data, get_user_with_id};
use crate::cookie_helpers::get_cookie_from_token;
use crate::dto::dive_session_dto::{DiveSessionModificationData, DiveSessionQueryData};
use crate::dto::user_auth_dto::{UserModificationData, UserQueryData};
use crate::graphql_schema::DbPool;
use crate::{actions::get_dive_session_by_id, diesel::ExpressionMethods};

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{result::Error, RunQueryDsl};
use tracing::info;

pub async fn update_user(
    ctx: &Context<'_>,
    new_password: Option<String>,
    user_mod_data: UserModificationData,
) -> UserQueryData {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let cookie =
        get_cookie_from_token(ctx).expect("there should be cookie data, as this route is guarded");

    let user_session = get_user_session_data(ctx, cookie.encoded_session_id)
        .await
        .expect("redis could fail here");

    let my_user_mod_data = user_mod_data.clone();
    let output_user = web::block(move || {
        let conn = pool_ctx.get().unwrap();
        use crate::schema::users::dsl::*;
        diesel::update(users)
            .filter(user_id.eq(&user_session.user_id))
            .set((&my_user_mod_data, updated_at.eq(Utc::now().naive_utc())))
            .execute(&conn)
    })
    .await
    .expect("web::block error here?");

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let updated_user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_user_with_id(&mut conn, &user_session.user_id)
    })
    .await
    .expect("web::block error here?")
    .expect("error getting session");

    info!("the output: {:?}", output_user);

    updated_user
}
