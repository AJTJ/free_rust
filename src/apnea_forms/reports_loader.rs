use super::actions::get_reports::get_reports;
use super::dto::report_dto::ReportsRetrievalData;
use crate::apnea_forms::dto::report_dto::Report;
use crate::graphql_schema::DbPool;
use crate::utility::errors::ActixBlockingSnafu;
use crate::utility::errors::BigError;
use actix_web::web;
use async_graphql::async_trait;
use async_graphql::dataloader::*;
use snafu::ResultExt;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ReportLoader(DbPool);

impl ReportLoader {
    pub fn new(postgres_pool: DbPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait::async_trait]
impl Loader<ReportsRetrievalData> for ReportLoader {
    type Value = Report;
    type Error = Arc<BigError>;

    async fn load(
        &self,
        keys: &[ReportsRetrievalData],
    ) -> Result<HashMap<ReportsRetrievalData, Self::Value>, Self::Error> {
        let pool = self.0.clone();
        let my_keys = keys.to_vec();
        let output = web::block(move || {
            let mut conn = pool.get().unwrap();
            get_reports(&mut conn, my_keys)
        })
        .await
        .context(ActixBlockingSnafu)??;

        let mut m: HashMap<ReportsRetrievalData, Report> = HashMap::new();
        if let Some(reports) = output {
            for report in reports.into_iter() {
                m.insert(ReportsRetrievalData::SessionId(report.session_id), report);
            }
        }
        Ok(m)
    }
}
