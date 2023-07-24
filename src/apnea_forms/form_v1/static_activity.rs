use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::{enums::StaticStoppingEnumV1, form::FormFieldOptionsV1};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticReportFieldV1Request")]
pub struct StaticReportFieldV1 {
    static_holds: Vec<StaticReportFieldsV1>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticReportFieldsV1Request")]
pub struct StaticReportFieldsV1 {
    time_goal: Option<NaiveTime>,
    time_achieved: Option<NaiveTime>,
    reason_for_stopping: Option<StaticStoppingEnumV1>,
    level_of_hypoxia: Option<i32>,
    level_of_relaxation: Option<i32>,
    activity_of_the_mind: Option<i32>,
    average_heart_rate: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticFormV1Request")]
pub struct StaticFormV1 {
    time_goal: Option<FormFieldOptionsV1>,
    time_achieved: Option<FormFieldOptionsV1>,
    reason_for_stopping: Option<FormFieldOptionsV1>,
    level_of_hypoxia: Option<FormFieldOptionsV1>,
    level_of_relaxation: Option<FormFieldOptionsV1>,
    activity_of_the_mind: Option<FormFieldOptionsV1>,
    average_heart_rate: Option<FormFieldOptionsV1>,
}
