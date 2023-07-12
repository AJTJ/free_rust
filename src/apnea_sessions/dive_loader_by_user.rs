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

pub struct DiveLoaderByUser(DbPool);

impl DiveLoaderByUser {
    pub fn new(postgres_pool: DbPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait::async_trait]
impl Loader<DiveRetrievalData> for DiveLoaderByUser {
    type Value = Dive;
    type Error = Arc<BigError>;

    async fn load(
        &self,
        keys: &[DiveRetrievalData],
    ) -> Result<HashMap<DiveRetrievalData, Self::Value>, Self::Error> {
        let pool = self.0.clone();
        let my_keys = keys.to_vec();
        let output = web::block(move || {
            let mut conn = pool.get().unwrap();
            get_dives(&mut conn, my_keys, None)
        })
        .await
        .context(ActixBlockingSnafu)??;

        let mut m: HashMap<DiveRetrievalData, Dive> = HashMap::new();
        if let Some(dives) = output {
            for dive in dives {
                m.insert(DiveRetrievalData::User(dive.user_id), dive);
            }
        }

        Ok(m)
    }
}
