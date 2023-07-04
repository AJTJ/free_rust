use crate::auth::dto::user_dto::{User, UserRetrievalData, UserUpdate};
use crate::diesel::ExpressionMethods;
use crate::graphql_schema::DbPool;
use crate::utility::errors::BigError;
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use uuid::Uuid;

use super::{get_user, get_user_id_from_auth};

pub async fn update_user(
    ctx: &Context<'_>,
    // TODO: Impl change password here? Or somewhere else?
    new_password: Option<String>,
    input_user_id: Option<Uuid>,
    user_mod_data: UserUpdate,
) -> Result<User, BigError> {
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    // NOTE: get user_id from cookie/session if it isn't included
    let input_user_id = match input_user_id {
        Some(u) => u,
        None => get_user_id_from_auth(ctx).await?,
    };

    let my_user_mod_data = user_mod_data.clone();

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        use crate::schema::users::dsl::{id as user_id, updated_at, users};
        diesel::update(users)
            .filter(user_id.eq(&input_user_id))
            .set((&my_user_mod_data, updated_at.eq(Utc::now())))
            .execute(&mut conn)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselUpdateError { source: e })?;

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        get_user(&mut conn, UserRetrievalData::Id(input_user_id))
    })
    .await
    .expect("web::block error here?")
    .map_err(|e| BigError::DieselQueryError { source: e })
}
