use super::{
    activity_modules::{DisciplineAndMaxDepthV1, MaxDepthV1},
    deep_dive::{DeepDiveFormV1, DeepDiveReportFieldV1, DeepDiveReportFieldsV1},
    dynamic::{DynamicFormV1, DynamicReportFieldV1},
    enums::{InjuryEnumV1, TemperatureEnumV1},
    general::{
        EaseOfEqualizationV1, EndTimeV1, GeneralFeelingV1, InjuryV1, LocationV1, SessionNameV1,
        VisibilityV1, WaterTempV1,
    },
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
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// REPORT

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "ReportV1Request")]
pub struct ReportV1 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveReportFieldV1>,
    dynamic_dives: Option<DynamicReportFieldV1>,
    static_holds: Option<StaticReportFieldV1>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<DisciplineAndMaxDepthV1>,
    max_depth: Option<MaxDepthV1>,
    // GENERAL
    start_time: StartTimeV1,
    session_name: Option<SessionNameV1>,
    end_time: Option<EndTimeV1>,
    ease_of_equalization: Option<EaseOfEqualizationV1>,
    visibility: Option<VisibilityV1>,
    general_feeling: Option<GeneralFeelingV1>,
    injury: Option<InjuryV1>,
    water_temp: Option<WaterTempV1>,
    location: Option<LocationV1>,
    // REPORT SPECIFIC
}

// FORM

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormFieldOptionsV1Request")]
pub struct FormFieldOptionsV1 {
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormV1Request")]
pub struct FormV1 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveFormV1>,
    dynamic_dives: Option<DynamicFormV1>,
    static_holds: Option<StaticFormV1>,
    // ACTIVITY-BASED
    // Deep Diving
    discipline_and_max_depth: Option<FormFieldOptionsV1>,
    max_depth: Option<FormFieldOptionsV1>,
    // GENERAL
    // NOTE: start_time is required in a report, even though it is optional here
    start_time: Option<FormFieldOptionsV1>,
    session_name: Option<FormFieldOptionsV1>,
    end_time: Option<FormFieldOptionsV1>,
    ease_of_equalization: Option<FormFieldOptionsV1>,
    visibility: Option<FormFieldOptionsV1>,
    general_feeling: Option<FormFieldOptionsV1>,
    injury: Option<FormFieldOptionsV1>,
    water_temp: Option<FormFieldOptionsV1>,
    location: Option<FormFieldOptionsV1>,
    // FORM SPECIFIC
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
