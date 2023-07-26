use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::{
    enums::{DisciplinesEnumV1, DynIncidentsEnumV1, TurnReasonsEnumV1},
    form::FormFieldOptionsV1,
};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynDisciplineV1Request")]
pub struct DynDisciplineV1 {
    discipline: Option<DisciplinesEnumV1>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynGoalDistanceV1Request")]
pub struct DynGoalDistanceV1 {
    distance: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynAchievedDistanceV1Request")]
pub struct DynAchievedDistanceV1 {
    distance: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynDiveTimeV1Request")]
pub struct DynDiveTimeV1 {
    time: Option<NaiveTime>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynEndReasonsV1Request")]
pub struct DynEndReasonsV1 {
    reasons: Option<Vec<TurnReasonsEnumV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynGeneralFeelingV1Request")]
pub struct DynGeneralFeelingV1 {
    value: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynIncidentsV1Request")]
pub struct DynIncidentsV1 {
    incidents: Option<Vec<DynIncidentsEnumV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicReportFieldsV1Request")]
pub struct DynamicReportFieldsV1 {
    discipline: Option<DynDisciplineV1>,
    goal_distance: Option<DynGoalDistanceV1>,
    achieved_distance: Option<DynAchievedDistanceV1>,
    dive_time: Option<DynDiveTimeV1>,
    reasons_for_ending: Option<DynEndReasonsV1>,
    general_feeling: Option<DynGeneralFeelingV1>,
    incidents: Option<DynIncidentsV1>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DynamicReportFieldV1Request")]
pub struct DynamicReportFieldV1 {
    dives: Option<Vec<DynamicReportFieldsV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
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

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}
