use crate::{
    apnea_forms::form_v1::unique_apneas::UniqueApneaActivity,
    apnea_sessions::dto::unique_apnea_dto::{UniqueApnea, UniqueApneaCreation, UniqueApneaInput},
    auth::actions::get_user_id_from_auth,
    graphql_schema::DbPool,
    utility::errors::{BigError, SerdeSerializeSnafu},
};
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use snafu::ResultExt;
use uuid::Uuid;

pub async fn insert_unique_apnea(
    ctx: &Context<'_>,
    apnea_session_id: Uuid,
    unique_apnea_input: UniqueApneaInput,
) -> Result<UniqueApnea, BigError> {
    let current_stamp = Utc::now();
    let user_id = get_user_id_from_auth(ctx).await?;
    let new_unique_apnea = UniqueApneaCreation {
        activity_data: serde_json::to_value(UniqueApneaActivity::from_input(
            unique_apnea_input.activity_data,
        ))
        .context(SerdeSerializeSnafu)?,

        session_id: apnea_session_id,
        user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::unique_apneas::dsl::unique_apneas;

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::insert_into(unique_apneas)
            .values(&new_unique_apnea)
            .get_result::<UniqueApnea>(&mut conn)
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })
}
