use super::unique_apnea_dto::{UniqueApnea, UniqueApneaRetrievalData};
use crate::{
    apnea_forms::{
        dto::form_dto::Form,
        form_loader::FormLoader,
        form_v1::{
            form::ReportV1,
            unique_apneas::{UniqueApneaActivity, UniqueApneaActivityRequest},
        },
        forms_interface::{ReportRequest, ReportResponse, StoredReport},
    },
    apnea_sessions::dive_loader_by_session::DiveLoaderBySession,
    schema::apnea_sessions,
    utility::errors::{BigError, DieselQuerySnafu},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde_json::Value;
use snafu::ResultExt;
use std::sync::Arc;
use uuid::Uuid;

#[derive(InputObject, Clone)]
pub struct ApneaSessionInput {
    pub report_data: ReportRequest,
    // pub unique_apnea_activities: Option<Vec<UniqueApneaActivityRequest>>,
    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_session_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = apnea_sessions)]
pub struct ApneaSessionCreation {
    // TODO: figure out how to make this the type, directly
    pub report_data: Value,

    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_session_id: Option<Uuid>,
    pub user_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

// Matches the database object 1:1
#[derive(Queryable, SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct ApneaSession {
    #[graphql(skip)]
    pub report_data: StoredReport,

    // relationships data
    #[graphql(skip)]
    pub form_id: Uuid,
    #[graphql(skip)]
    pub original_form_id: Option<Uuid>,

    // NOTE: what purpose does previous_session_id serve?
    #[graphql(skip)]
    pub previous_session_id: Option<Uuid>,
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
impl ApneaSession {
    // TODO: This returns the form, but we will want to handle "original_form"
    async fn form(&self, ctx: &Context<'_>) -> Result<Option<Form>, Arc<BigError>> {
        let form_response = ctx
            .data_unchecked::<DataLoader<FormLoader>>()
            .load_one(self.form_id)
            .await;

        form_response
    }

    // The question here is:
    // As I gather all the unique_apneas, how do I store the order of the unique apneas?
    // It seems like the stored_report might want to store the information of WHERE the unique apneas are ordered
    // But it's starting to seem like perhaps this is UN-normalized behavior
    // I might want to forego the `isActive` and `field_order` information on report fields, entirely
    // since a report is ALWAYS associated with a form.
    // I think re-thinking the normalization of my forms might help what I'm doing
    // I don't think there's anything necessarily wrong with what I'm doing, but I do think it it is entirely an optimization
    // And I will probably rethink "optimizing" any further
    async fn report(&self, ctx: &Context<'_>) -> Result<ReportResponse, Arc<BigError>> {
        let unique_apneas = ctx
            .data_unchecked::<DataLoader<DiveLoaderBySession>>()
            .load_one(UniqueApneaRetrievalData::Session(self.id))
            .await?;

        // TODO Does this require data loaders for the unique apneas?
        let report = match &self.report_data {
            StoredReport::V1(report) => {
                let mut report: ReportV1 = report.clone().into();
                if let Some(apneas) = unique_apneas {
                    for apnea in apneas.iter() {
                        match &apnea.activity_data {
                            UniqueApneaActivity::DeepDiveV1(deep) => report
                                .deep_dives
                                .get_or_insert(vec![deep.clone()])
                                .push(deep.clone()),
                            UniqueApneaActivity::DynDiveV1(dynamic) => report
                                .dynamic_dives
                                .get_or_insert(vec![dynamic.clone()])
                                .push(dynamic.clone()),
                            UniqueApneaActivity::StaticHoldsV1(sta) => report
                                .static_holds
                                .get_or_insert(vec![sta.clone()])
                                .push(sta.clone()),
                        }
                    }
                };
                ReportResponse::V1(report)
            }
        };

        Ok(report)
    }

    async fn unique_apneas(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<Vec<UniqueApnea>>, Arc<BigError>> {
        ctx.data_unchecked::<DataLoader<DiveLoaderBySession>>()
            .load_one(UniqueApneaRetrievalData::Session(self.id))
            .await
    }
}

pub enum ApneaSessionRetrievalData {
    Sessions(Vec<Uuid>),
    User(Uuid),
}
