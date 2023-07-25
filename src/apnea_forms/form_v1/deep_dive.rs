use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::{
    enums::{
        DeepDiveIncidentsEnumV1, DeepDiveSensationsV1, DisciplinesEnumV1, InjuryEnumV1,
        TurnReasonsEnumV1,
    },
    form::FormFieldOptionsV1,
};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveReportFieldV1Request")]
pub struct DeepDiveReportFieldV1 {
    dives: Option<Vec<DeepDiveReportFieldsV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveReportFieldsV1Request")]
pub struct DeepDiveReportFieldsV1 {
    discipline: Option<DisciplinesEnumV1>,
    goal_depth: Option<i32>,
    achieved_depth: Option<i32>,
    dive_time: Option<NaiveTime>,
    early_turn_depth: Option<i32>,
    reason_for_turning: Option<Vec<TurnReasonsEnumV1>>,
    general_feeling: Option<i32>,
    specific_sensations: Option<Vec<DeepDiveSensationsV1>>,
    thoughts_quality: Option<i32>,
    injuries: Option<Vec<InjuryEnumV1>>,
    other_incidents: Option<Vec<DeepDiveIncidentsEnumV1>>,
    mouth_fill_depth: Option<i32>,
    mouth_fill_charge_depths: Option<Vec<i32>>,
    turn_quality: Option<i32>,
    level_of_hypoxia: Option<i32>,
    level_of_exertion: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveFormV1Request")]
pub struct DeepDiveFormV1 {
    discipline: Option<FormFieldOptionsV1>,
    goal_depth: Option<FormFieldOptionsV1>,
    achieved_depth: Option<FormFieldOptionsV1>,
    dive_time: Option<FormFieldOptionsV1>,
    early_turn_depth: Option<FormFieldOptionsV1>,
    reason_for_turning: Option<FormFieldOptionsV1>,
    general_feeling: Option<FormFieldOptionsV1>,
    specific_sensations: Option<FormFieldOptionsV1>,
    thoughts: Option<FormFieldOptionsV1>,
    incidents: Option<FormFieldOptionsV1>,
    mouth_fill_depth: Option<FormFieldOptionsV1>,
    mouth_fill_charge_depths: Option<FormFieldOptionsV1>,
    turn_quality: Option<FormFieldOptionsV1>,
    level_of_hypoxia: Option<FormFieldOptionsV1>,
    level_of_exertion: Option<FormFieldOptionsV1>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}
