use super::enums::{DisciplinesEnum, WildlifeEnumV1};
use crate::{
    apnea_forms::{
        actions::{
            archive_form::archive_form, archive_report::archive_report, insert_form::insert_form,
            insert_report::insert_report,
        },
        dto::{
            form_dto::{Form, FormDetails},
            report_dto::{Report, ReportDetails},
        },
        helpers::FormResponse,
    },
    utility::errors::BigError,
};
use async_graphql::{Context, InputObject, OneofObject, SimpleObject, Union};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

// Report Name

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
struct SessionNameV1 {
    name: Option<String>,
    // defaults
    field_order: Option<i32>,
}

// Wildlife

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
struct WildlifeV1 {
    value: Option<WildlifeEnumV1>,
    // defaults
    field_order: Option<i32>,
}

// Weather

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
struct WeatherV1 {
    wind: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

// Discipline and Max Depth
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
struct InnerDisciplineMaxDepthV1 {
    discipline: Option<DisciplinesEnum>,
    max_depth: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
struct DisciplineAndMaxDepthV1 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthV1>>,
    // defaults
    field_order: Option<i32>,
}

// MAX DEPTH

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
struct MaxDepthV1 {
    max_depth: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

// CONGESTION

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
struct CongestionV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

// VISIBILITY

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
struct VisibilityV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

// FORMS

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
pub struct FormV1 {
    session_name: Option<SessionNameV1>,
    wildlife: Option<WildlifeV1>,
    weather: Option<WeatherV1>,
    discipline_and_max_depth: Option<DisciplineAndMaxDepthV1>,
    max_depth: Option<MaxDepthV1>,
    congestion: Option<CongestionV1>,
    visibility: Option<VisibilityV1>,
}

#[derive(OneofObject, Serialize, Deserialize, Debug, Clone)]
enum RequestFieldsV1 {
    SessionNameV1(SessionNameV1),
    WildlifeV1(WildlifeV1),
}

#[derive(Union, Serialize, Deserialize, Debug, Clone)]
enum ResponseFieldV1 {
    SessionNameV1(SessionNameV1),
    WildlifeV1(WildlifeV1),
}

pub struct NewFormV1(Vec<ResponseFieldV1>);

impl NewFormV1 {}

// Logic

impl FormV1 {
    pub async fn insert_form(
        &self,
        ctx: &Context<'_>,
        form_details: FormDetails,
        user_id: &Uuid,
    ) -> Result<Option<Form>, BigError> {
        // TODO: perform validation
        let form = insert_form(ctx, form_details, FormResponse::V1(self.clone()), user_id).await?;
        Ok(form)
    }

    pub async fn modify_form(
        &self,
        ctx: &Context<'_>,
        previous_form_id: &Uuid,
        form_details: FormDetails,
        user_id: &Uuid,
    ) -> Result<Option<Form>, BigError> {
        archive_form(ctx, previous_form_id, user_id).await?;
        let form = insert_form(ctx, form_details, FormResponse::V1(self.clone()), user_id).await?;
        Ok(form)
    }

    pub async fn insert_report(
        &self,
        ctx: &Context<'_>,
        session_id: &Uuid,
        report_details: ReportDetails,
        user_id: &Uuid,
    ) -> Result<Option<Report>, BigError> {
        let report = insert_report(
            ctx,
            session_id,
            report_details,
            FormResponse::V1(self.clone()),
            user_id,
        )
        .await?;
        Ok(report)
    }

    pub async fn modify_report(
        &self,
        ctx: &Context<'_>,
        session_id: &Uuid,
        previous_report_id: &Uuid,
        report_details: ReportDetails,
        user_id: &Uuid,
    ) -> Result<Option<Report>, BigError> {
        archive_report(ctx, previous_report_id, user_id).await?;
        let report = insert_report(
            ctx,
            session_id,
            report_details,
            FormResponse::V1(self.clone()),
            user_id,
        )
        .await?;
        Ok(report)
    }
}
