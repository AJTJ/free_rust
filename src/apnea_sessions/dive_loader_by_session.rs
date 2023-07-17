use super::actions::get_dives;
use super::dto::dive_dto::Dive;
use super::dto::dive_dto::DiveRetrievalData;
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
impl Loader<DiveRetrievalData> for DiveLoaderBySession {
    type Value = Vec<Dive>;
    type Error = Arc<BigError>;

    // TODO: Could this be more efficient, if I am getting dives by session_id, then I should be able to already sort them?
    async fn load(
        &self,
        keys: &[DiveRetrievalData],
    ) -> Result<HashMap<DiveRetrievalData, Self::Value>, Self::Error> {
        let pool = self.0.clone();
        let my_keys = keys.to_vec();
        let output = web::block(move || {
            let mut conn = pool.get().unwrap();
            get_dives(&mut conn, my_keys)
        })
        .await
        .context(ActixBlockingSnafu)??;

        let mut m: HashMap<DiveRetrievalData, Vec<Dive>> = HashMap::new();
        if let Some(dives) = output {
            for dive in dives {
                m.entry(DiveRetrievalData::Session(dive.session_id))
                    .and_modify(|e| e.push(dive.clone()))
                    .or_insert(vec![dive]);
            }
        }
        Ok(m)
    }
}

// m.insert(DiveRetrievalData::Session(dive.session_id), dive);
