use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    activity_modules::DisciplineAndMaxDepthV2,
    deep_dive::{DeepDiveFormV2, DeepDiveReportFieldsV2},
    dynamic::{DynamicFormV2, DynamicReportFieldV2},
    static_activity::{StaticFormV2, StaticReportFieldV2},
};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StartTimeV2Request")]
struct StartTimeV2 {
    time: DateTime<Utc>,
    // defaults
    field_order: Option<i32>,
}
// End Time

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "EndTimeV2Request")]
struct EndTimeV2 {
    time: Option<DateTime<Utc>>,
    // defaults
    field_order: Option<i32>,
}

// REPORT

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "ReportV2Request")]
pub struct ReportV2 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveReportFieldsV2>,
    dynamic_dives: Option<DynamicReportFieldV2>,
    static_holds: Option<StaticReportFieldV2>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<DisciplineAndMaxDepthV2>,
    // GENERAL
    end_time: Option<EndTimeV2>,
    // REPORT SPECIFIC
    start_time: StartTimeV2,
}

// FORM

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormV2Request")]
pub struct FormV2 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveFormV2>,
    dynamic_dives: Option<DynamicFormV2>,
    static_holds: Option<StaticFormV2>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<FormFieldOptionsV2>,
    // GENERAL
    end_time: Option<FormFieldOptionsV2>,
    // FORM SPECIFIC
    form_name: String,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormFieldOptionsV2Request")]
pub struct FormFieldOptionsV2 {
    field_order: i32,
}
// TODO: write a simple test to ensure that every field in a form is in a report and vice versa?
