use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::{
    enums::{DisciplinesEnumV1, DynamicIncidentsEnumV1, TurnReasonsEnumV1},
    form::FormFieldOptionsV1,
};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicReportFieldV1Request")]
pub struct DynamicReportFieldV1 {
    dynamic_dives: Vec<DynamicReportFieldsV1>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicReportFieldsV1Request")]
pub struct DynamicReportFieldsV1 {
    discipline: Option<DisciplinesEnumV1>,
    goal_distance: Option<i32>,
    achieved_distance: Option<i32>,
    dive_time: Option<NaiveTime>,
    reason_for_ending: Option<Vec<TurnReasonsEnumV1>>,
    general_feeling: Option<i32>,
    incidents: Option<Vec<DynamicIncidentsEnumV1>>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicFormV1Request")]
pub struct DynamicFormV1 {
    time_goal: Option<FormFieldOptionsV1>,
    time_achieved: Option<FormFieldOptionsV1>,
    reason_for_stopping: Option<FormFieldOptionsV1>,
    level_of_hypoxia: Option<FormFieldOptionsV1>,
    level_of_relaxation: Option<FormFieldOptionsV1>,
    activity_of_the_mind: Option<FormFieldOptionsV1>,
    average_heart_rate: Option<FormFieldOptionsV1>,
}
