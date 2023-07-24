use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::{
    enums::{DisciplinesEnumV2, DynamicIncidentsEnumV2, TurnReasonsEnumV2},
    form::FormFieldOptionsV2,
};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicReportFieldV2Request")]
pub struct DynamicReportFieldV2 {
    dynamic_dives: Vec<DynamicReportFieldsV2>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicReportFieldsV2Request")]
pub struct DynamicReportFieldsV2 {
    discipline: Option<DisciplinesEnumV2>,
    goal_distance: Option<i32>,
    achieved_distance: Option<i32>,
    dive_time: Option<NaiveTime>,
    reason_for_ending: Option<Vec<TurnReasonsEnumV2>>,
    general_feeling: Option<i32>,
    incidents: Option<Vec<DynamicIncidentsEnumV2>>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicFormV2Request")]
pub struct DynamicFormV2 {
    time_goal: Option<FormFieldOptionsV2>,
    time_achieved: Option<FormFieldOptionsV2>,
    reason_for_stopping: Option<FormFieldOptionsV2>,
    level_of_hypoxia: Option<FormFieldOptionsV2>,
    level_of_relaxation: Option<FormFieldOptionsV2>,
    activity_of_the_mind: Option<FormFieldOptionsV2>,
    average_heart_rate: Option<FormFieldOptionsV2>,
}
