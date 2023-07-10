use crate::{
    apnea_forms::dto::form_dto::Form, diesel::ExpressionMethods, graphql_schema::DbPool,
    utility::errors::BigError,
};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn archive_form(
    ctx: &Context<'_>,
    form_id_input: &Uuid,
    input_user_id: &Uuid,
) -> Result<Option<Form>, BigError> {
    use crate::schema::forms::dsl::{archived_at, archived_by, forms, id, is_active};

    let current_stamp = Utc::now();
    let my_form_id = form_id_input.clone();
    let my_user_id = input_user_id.clone();
    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    let archived_form = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::update(forms)
            .filter(id.eq(&my_form_id))
            .set((
                is_active.eq(false),
                archived_at.eq(current_stamp),
                archived_by.eq(my_user_id),
            ))
            .get_result(&mut conn)
            .optional()
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    Ok(archived_form)
}
