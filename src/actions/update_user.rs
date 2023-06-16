use crate::actions::{get_user_session_data, get_user_with_id};
use crate::diesel::ExpressionMethods;
use crate::dto::user_dto::{UserModificationData, UserQueryData};
use crate::graphql_schema::DbPool;
use crate::helpers::cookie_helpers::get_cookie_from_token;

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use tracing::info;
use uuid::Uuid;

pub async fn update_user(
    ctx: &Context<'_>,
    // TODO: Impl change password here?
    new_password: Option<String>,
    user_id: Option<Uuid>,
    user_mod_data: UserModificationData,
) -> UserQueryData {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    // get user_id from cookie/session
    let user_id = match user_id {
        Some(uuid) => uuid,
        None => {
            let cookie =
                get_cookie_from_token(ctx).expect("there should be cookie data, in update_user");

            let user_session = get_user_session_data(ctx, cookie.encoded_session_id)
                .await
                .expect("redis could fail here");

            user_session.user_id
        }
    };

    let my_user_mod_data = user_mod_data.clone();
    let output_user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        use crate::schema::users::dsl::{id as user_id, updated_at, users};
        diesel::update(users)
            .filter(user_id.eq(&user_id))
            .set((&my_user_mod_data, updated_at.eq(Utc::now().naive_utc())))
            .execute(&mut conn)
    })
    .await
    .expect("web::block error here?");

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let updated_user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_user_with_id(&mut conn, &user_id)
    })
    .await
    .expect("web::block error here?")
    .expect("error getting session");

    info!("the output: {:?}", output_user);

    updated_user
}
