use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    actions::get_dive_sessions_by_id, dto::dive_session_dto::DiveSession, errors::BigError,
    graphql_schema::DbPool,
};
use actix_web::web;
use async_graphql::async_trait;
use async_graphql::dataloader::*;
use async_graphql::Error;
use serde_json::to_vec;
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
    type Error = Arc<BigError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pool = self.0.clone();
        let my_keys = keys.to_vec();
        let output = web::block(move || {
            let mut conn = pool.get().unwrap();
            get_dive_sessions_by_id(&mut conn, &my_keys[..])
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselInsertError { source: e })?;

        // it seems like it is required to return a hashmap?
        let mut m: HashMap<Uuid, DiveSession> = HashMap::new();
        for d in output.into_iter() {
            m.insert(d.id, d);
        }
        Ok(m)
    }
}
