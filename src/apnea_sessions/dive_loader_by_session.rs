use super::actions::get_unique_apneas;
use super::dto::unique_apnea_dto::UniqueApnea;
use super::dto::unique_apnea_dto::UniqueApneaRetrievalData;
use crate::graphql_schema::DbPool;
use crate::utility::errors::ActixBlockingSnafu;
use crate::utility::errors::BigError;
use actix_web::web;
use async_graphql::async_trait;
use async_graphql::dataloader::*;
use snafu::ResultExt;
use std::collections::HashMap;
use std::sync::Arc;

pub struct DiveLoaderBySession(DbPool);

impl DiveLoaderBySession {
    pub fn new(postgres_pool: DbPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait::async_trait]
impl Loader<UniqueApneaRetrievalData> for DiveLoaderBySession {
    type Value = Vec<UniqueApnea>;
    type Error = Arc<BigError>;

    // TODO: Could this be more efficient, if I am getting dives by session_id, then I should be able to already sort them?
    async fn load(
        &self,
        keys: &[UniqueApneaRetrievalData],
    ) -> Result<HashMap<UniqueApneaRetrievalData, Self::Value>, Self::Error> {
        let pool = self.0.clone();
        let my_keys = keys.to_vec();
        let output = web::block(move || {
            let mut conn = pool.get().unwrap();
            get_unique_apneas(&mut conn, my_keys)
        })
        .await
        .context(ActixBlockingSnafu)??;

        let mut m: HashMap<UniqueApneaRetrievalData, Vec<UniqueApnea>> = HashMap::new();
        if let Some(dives) = output {
            for dive in dives {
                m.entry(UniqueApneaRetrievalData::Session(dive.session_id))
                    .and_modify(|e| e.push(dive.clone()))
                    .or_insert(vec![dive]);
            }
        }
        Ok(m)
    }
}

// m.insert(DiveRetrievalData::Session(dive.session_id), dive);
