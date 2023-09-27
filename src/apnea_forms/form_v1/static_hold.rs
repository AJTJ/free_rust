use super::{enums::StaticStoppingEnumV1, form::FormFieldOptionsV1};
use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticTimeGoalV1Request")]
pub struct StaticTimeGoalV1 {
    time: Option<NaiveTime>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticTimeAchievedV1Request")]
pub struct StaticTimeAchievedV1 {
    time: Option<NaiveTime>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticStoppingV1Request")]
pub struct StaticStoppingV1 {
    reason: Option<StaticStoppingEnumV1>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticHypoxiaV1Request")]
pub struct StaticHypoxiaV1 {
    value: Option<i32>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticRelaxationV1Request")]
pub struct StaticRelaxationV1 {
    value: Option<i32>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticMindV1Request")]
pub struct StaticMindV1 {
    value: Option<i32>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticHeartRateV1Request")]
pub struct StaticHeartRateV1 {
    value: Option<i32>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

// GROUPINGS

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticReportFieldV1Request")]
pub struct StaticReportFieldV1 {
    pub static_holds: Option<Vec<StaticReportFieldsV1>>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticReportFieldsV1Request")]
pub struct StaticReportFieldsV1 {
    time_goal: Option<StaticTimeGoalV1>,
    time_achieved: Option<StaticTimeAchievedV1>,
    reason_for_stopping: Option<StaticStoppingV1>,
    level_of_hypoxia: Option<StaticHypoxiaV1>,
    level_of_relaxation: Option<StaticRelaxationV1>,
    activity_of_the_mind: Option<StaticMindV1>,
    average_heart_rate: Option<StaticHeartRateV1>,
    // TODO: add exhale
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
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
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}
