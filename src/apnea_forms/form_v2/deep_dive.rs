use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::{
    enums::{
        DeepDiveIncidentsEnumV2, DeepDiveSensationsV2, DisciplinesEnumV2, InjuryEnumV2,
        TurnReasonsEnumV2,
    },
    form::FormFieldOptionsV2,
};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
pub struct DeepDiveReportFieldV2 {
    dives: Vec<DeepDiveReportFieldsV2>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveReportV2Request")]
pub struct DeepDiveReportFieldsV2 {
    discipline: Option<DisciplinesEnumV2>,
    goal_depth: Option<i32>,
    achieved_depth: Option<i32>,
    dive_time: Option<NaiveTime>,
    early_turn_depth: Option<i32>,
    reason_for_turning: Option<Vec<TurnReasonsEnumV2>>,
    general_feeling: Option<i32>,
    specific_sensations: Option<Vec<DeepDiveSensationsV2>>,
    thoughts_quality: Option<i32>,
    injuries: Option<Vec<InjuryEnumV2>>,
    other_incidents: Option<Vec<DeepDiveIncidentsEnumV2>>,
    mouth_fill_depth: Option<i32>,
    mouth_fill_charge_depths: Option<Vec<i32>>,
    turn_quality: Option<i32>,
    level_of_hypoxia: Option<i32>,
    level_of_exertion: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveFormV2Request")]
pub struct DeepDiveFormV2 {
    discipline: Option<FormFieldOptionsV2>,
    goal_depth: Option<FormFieldOptionsV2>,
    achieved_depth: Option<FormFieldOptionsV2>,
    dive_time: Option<FormFieldOptionsV2>,
    early_turn_depth: Option<FormFieldOptionsV2>,
    reason_for_turning: Option<FormFieldOptionsV2>,
    general_feeling: Option<FormFieldOptionsV2>,
    specific_sensations: Option<FormFieldOptionsV2>,
    thoughts: Option<FormFieldOptionsV2>,
    incidents: Option<FormFieldOptionsV2>,
    mouth_fill_depth: Option<FormFieldOptionsV2>,
    mouth_fill_charge_depths: Option<FormFieldOptionsV2>,
    turn_quality: Option<FormFieldOptionsV2>,
    level_of_hypoxia: Option<FormFieldOptionsV2>,
    level_of_exertion: Option<FormFieldOptionsV2>,
}
