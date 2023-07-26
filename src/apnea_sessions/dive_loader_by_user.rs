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

pub struct DiveLoaderByUser(DbPool);

impl DiveLoaderByUser {
    pub fn new(postgres_pool: DbPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait::async_trait]
impl Loader<UniqueApneaRetrievalData> for DiveLoaderByUser {
    type Value = UniqueApnea;
    type Error = Arc<BigError>;

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

        let mut m: HashMap<UniqueApneaRetrievalData, UniqueApnea> = HashMap::new();
        if let Some(dives) = output {
            for dive in dives {
                m.insert(UniqueApneaRetrievalData::User(dive.user_id), dive);
            }
        }

        Ok(m)
    }
}
