use super::{
    activity_modules::DisciplineAndMaxDepthV1,
    deep_dive::{DeepDiveFormV1, DeepDiveReportFieldsV1},
    dynamic::{DynamicFormV1, DynamicReportFieldV1},
    static_activity::{StaticFormV1, StaticReportFieldV1},
};
use crate::{
    apnea_forms::{
        actions::{archive_form::archive_form, insert_form::insert_form},
        dto::form_dto::{Form, FormDetails},
        forms_interface::FormResponse,
    },
    utility::errors::BigError,
};
use async_graphql::{Context, InputObject, OneofObject, SimpleObject, Union};
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    pg::{Pg, PgValue},
    sql_types,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StartTimeV1Request")]
struct StartTimeV1 {
    time: DateTime<Utc>,
    // defaults
    field_order: Option<i32>,
}
// End Time

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "EndTimeV1Request")]
struct EndTimeV1 {
    time: Option<DateTime<Utc>>,
    // defaults
    field_order: Option<i32>,
}

// REPORT

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "ReportV1Request")]
pub struct ReportV1 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveReportFieldsV1>,
    dynamic_dives: Option<DynamicReportFieldV1>,
    static_holds: Option<StaticReportFieldV1>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<DisciplineAndMaxDepthV1>,
    // GENERAL
    end_time: Option<EndTimeV1>,
    // REPORT SPECIFIC
    start_time: StartTimeV1,
}

// FORM

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormFieldOptionsV1Request")]
pub struct FormFieldOptionsV1 {
    field_order: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormV1Request")]
pub struct FormV1 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveFormV1>,
    dynamic_dives: Option<DynamicFormV1>,
    static_holds: Option<StaticFormV1>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<FormFieldOptionsV1>,
    // GENERAL
    end_time: Option<FormFieldOptionsV1>,
    // FORM SPECIFIC
    form_name: String,
}

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
}

// TODO: write a simple test to ensure that every field in a form is in a report and vice versa?
