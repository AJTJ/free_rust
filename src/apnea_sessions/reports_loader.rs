use super::actions::get_apnea_sessions;
use crate::apnea_forms::actions::get_reports::get_reports;
use crate::apnea_forms::dto::report_dto::Report;
use crate::apnea_forms::dto::report_dto::ReportsRetrievalData;
use crate::graphql_schema::DbPool;
use crate::utility::errors::ActixBlockingSnafu;
use crate::utility::errors::BigError;
use crate::utility::gql::query_dto::QueryParams;
use actix_web::web;
use async_graphql::async_trait;
use async_graphql::dataloader::*;
use snafu::ResultExt;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct ReportsLoader(DbPool);

impl ReportsLoader {
    pub fn new(postgres_pool: DbPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ReportsLoader {
    type Value = Report;
    type Error = Arc<BigError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pool = self.0.clone();
        let my_keys = keys.to_vec();
        let output = web::block(move || {
            let mut conn = pool.get().unwrap();
            get_reports(
                &mut conn,
                ReportsRetrievalData::ReportIds(my_keys),
                // TODO: I don't know if this makes sense
                QueryParams {
                    after: None,
                    first: None,
                },
            )
        })
        .await
        .context(ActixBlockingSnafu)??;

        // TODO: Not sure this makes sense either
        let mut m: HashMap<Uuid, Report> = HashMap::new();
        for edge in output.edges.into_iter() {
            m.insert(edge.node.id, edge.node);
        }
        Ok(m)
    }
}
