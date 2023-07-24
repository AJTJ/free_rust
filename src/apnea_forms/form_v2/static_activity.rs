use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::{enums::StaticStoppingEnumV2, form::FormFieldOptionsV2};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticReportFieldV2Request")]
pub struct StaticReportFieldV2 {
    static_holds: Vec<StaticReportFieldsV2>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticReportFieldsV2Request")]
pub struct StaticReportFieldsV2 {
    time_goal: Option<NaiveTime>,
    time_achieved: Option<NaiveTime>,
    reason_for_stopping: Option<StaticStoppingEnumV2>,
    level_of_hypoxia: Option<i32>,
    level_of_relaxation: Option<i32>,
    activity_of_the_mind: Option<i32>,
    average_heart_rate: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticFormV2Request")]
pub struct StaticFormV2 {
    time_goal: Option<FormFieldOptionsV2>,
    time_achieved: Option<FormFieldOptionsV2>,
    reason_for_stopping: Option<FormFieldOptionsV2>,
    level_of_hypoxia: Option<FormFieldOptionsV2>,
    level_of_relaxation: Option<FormFieldOptionsV2>,
    activity_of_the_mind: Option<FormFieldOptionsV2>,
    average_heart_rate: Option<FormFieldOptionsV2>,
}
