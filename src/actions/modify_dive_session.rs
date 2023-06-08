use crate::diesel::ExpressionMethods;
use crate::dto::db_query_dto::DBQueryObject;
use crate::dto::dive_session_dto::{
    DiveSessionCreationData, DiveSessionInputData, DiveSessionModificationData,
    DiveSessionQueryData, DiveSessionQueryInput,
};
use crate::graphql_schema::DbPool;

use actix_web::web;
use async_graphql::{Context, Error};
use chrono::Utc;
use diesel::RunQueryDsl;
use tracing::info;

pub async fn modify_dive_session(ctx: &Context<'_>, session_mod_data: DiveSessionModificationData)
/* -> Result<DiveSessionQueryData, Error> */
{
    let current_stamp = Utc::now().naive_utc();

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    let output_dive_session = web::block(move || {
        let conn = pool_ctx.get().unwrap();
        use crate::schema::dive_sessions::dsl::*;
        diesel::update(dive_sessions)
            .filter(session_id.eq(&session_mod_data.session_id))
            .set(&session_mod_data)
            .execute(&conn)
        // .get_result(&conn);
    })
    .await
    .expect("web::block error here?");

    info!("the output: {:?}", output_dive_session);

    // Ok(output_dive_session)

    // unimplemented!()
}
