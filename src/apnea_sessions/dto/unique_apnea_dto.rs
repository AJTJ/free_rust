use super::apnea_session_dto::ApneaSession;
use crate::{
    apnea_forms::form_v1::unique_apneas::{UniqueApneaActivity, UniqueApneaActivityRequest},
    apnea_sessions::actions::get_apnea_session,
    graphql_schema::DbPool,
    schema::unique_apneas,
    utility::{errors::BigError, gql::query_dto::QueryParams},
};
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, OneofObject, SimpleObject};
use chrono::{DateTime, NaiveTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(InputObject)]
pub struct UniqueApneaInput {
    pub activity_data: UniqueApneaActivityRequest,
}

#[derive(Insertable)]
#[diesel(table_name = unique_apneas)]
pub struct UniqueApneaCreation {
    pub activity_data: Value,

    pub session_id: Uuid,
    pub user_id: Uuid,
    // partial default data
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

// Matches the database object order 1:1
#[derive(Queryable, SimpleObject, Clone)]
pub struct UniqueApnea {
    pub activity_data: UniqueApneaActivity,

    #[graphql(skip)]
    pub session_id: Uuid,
    #[graphql(skip)]
    pub user_id: Uuid,

    // default data
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    #[graphql(skip)]
    pub archived_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub archived_by: Option<Uuid>,
}

#[ComplexObject]
impl UniqueApnea {
    async fn apnea_session(
        &self,
        ctx: &Context<'_>,
        query_params: Option<QueryParams>,
    ) -> Result<ApneaSession, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let session_id = self.session_id;
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_apnea_session(&mut conn, &session_id).map(ApneaSession::from)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum UniqueApneaRetrievalData {
    Session(Uuid),
    User(Uuid),
}

// pub discipline_type: Option<String>,
//     pub depth: Option<f64>,
//     pub distance: Option<f64>,
//     pub dive_time: Option<i64>,
//     pub dive_name: Option<String>,
