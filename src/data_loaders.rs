use std::collections::HashMap;

use crate::{
    actions::get_dive_sessions_by_id, dto::dive_session_dto::DiveSession, errors::BigError,
    graphql_schema::DbPool,
};
use actix_web::web;
use async_graphql::async_trait;
use async_graphql::dataloader::*;
use uuid::Uuid;

pub struct DiveSessionsLoader(DbPool);

impl DiveSessionsLoader {
    pub fn new(postgres_pool: DbPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for DiveSessionsLoader {
    type Value = DiveSession;
    type Error = BigError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let output = web::block(move || {
            let mut conn = self.0.get().unwrap();
            get_dive_sessions_by_id(&mut conn, keys)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselInsertError { source: e });

        // the problem here is that output is a Result<VecDiveSession ...>
        // and not the HashMap that i think is required
        output
    }
}
