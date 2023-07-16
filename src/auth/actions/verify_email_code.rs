use crate::auth::actions::modify_user;
use crate::auth::dto::user_dto::{User, UserUpdate};
use crate::graphql_schema::DbPool;
use crate::utility::errors::BigError;
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn verify_email_code(
    ctx: &Context<'_>,
    unverified_user_id: &Uuid,
    unverified_email: &String,
    incoming_verification_code: &String,
) -> Result<User, BigError> {
    use crate::schema::users::dsl::{
        email, id as user_id, users, verification_code, verification_code_expiry,
    };
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let my_unverified_user_id = unverified_user_id.clone();
    let my_unverified_email = unverified_email.clone();
    let my_incoming_verification_code = incoming_verification_code.clone();
    let user = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        users
            .filter(
                user_id.eq(my_unverified_user_id).and(
                    email
                        .eq(my_unverified_email)
                        .and(verification_code.eq(my_incoming_verification_code))
                        .and(verification_code_expiry.lt(Utc::now())),
                ),
            )
            .get_result::<User>(&mut conn)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselUpdateError { source: e })?;

    let verified_update = UserUpdate {
        last_login: None,
        username: None,
        email: None,
        is_active: None,
        is_email_verified: Some(true),
        verification_code: None,
        verification_code_expiry: None,
    };

    modify_user(ctx, None, Some(&user.id), verified_update).await?;

    Ok(user)
}
