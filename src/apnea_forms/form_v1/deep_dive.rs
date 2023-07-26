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

// Discipline

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDisciplineV1Request")]
pub struct DeepDisciplineV1 {
    discipline: Option<DisciplinesEnumV1>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Goal Depth

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepGoalDepthV1Request")]
pub struct DeepGoalDepthV1 {
    depth: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Achieved Depth

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepAchievedDepthV1Request")]
pub struct DeepAchievedDepthV1 {
    depth: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Dive Time

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveTimeV1Request")]
pub struct DeepDiveTimeV1 {
    time: Option<NaiveTime>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Early Turn Depth

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepEarlyTurnDepthV1Request")]
pub struct DeepEarlyTurnDepthV1 {
    time: Option<NaiveTime>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Reason For Turning

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepTurnReasonsV1Request")]
pub struct DeepTurnReasonsV1 {
    reasons: Option<Vec<TurnReasonsEnumV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// General Feeling
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepGeneralFeelingV1Request")]
pub struct DeepGeneralFeelingV1 {
    value: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Sensations
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepSensationsV1Request")]
pub struct DeepSensationsV1 {
    sensations: Option<Vec<DeepDiveSensationsV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Mental Calm
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "MentalCalmV1Request")]
pub struct MentalCalmV1 {
    value: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Injuries
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepInjuriesV1Request")]
pub struct DeepInjuriesV1 {
    injuries: Option<Vec<InjuryEnumV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Incidents
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepIncidentsV1Request")]
pub struct DeepIncidentsV1 {
    incidents: Option<Vec<DeepDiveIncidentsEnumV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Mouth Fill Charge Depths
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "MouthFillChargeDepthsV1Request")]
pub struct MouthFillChargeDepthsV1 {
    depths: Option<Vec<i32>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Turn Quality
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepTurnQualityV1Request")]
pub struct DeepTurnQualityV1 {
    value: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Hypoxia
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepHypoxiaV1Request")]
pub struct DeepHypoxiaV1 {
    value: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// Exertion
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepExertionV1Request")]
pub struct DeepExertionV1 {
    value: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveReportFieldsV1Request")]
pub struct DeepDiveReportFieldsV1 {
    discipline: Option<DeepDisciplineV1>,
    goal_depth: Option<DeepGoalDepthV1>,
    achieved_depth: Option<DeepAchievedDepthV1>,
    dive_time: Option<DeepDiveTimeV1>,
    early_turn_depth: Option<DeepEarlyTurnDepthV1>,
    reason_for_turning: Option<DeepTurnReasonsV1>,
    general_feeling: Option<DeepGeneralFeelingV1>,
    sensations: Option<DeepSensationsV1>,
    mental_calm: Option<MentalCalmV1>,
    injuries: Option<DeepInjuriesV1>,
    other_incidents: Option<DeepIncidentsV1>,
    mouth_fill_charge_depths: Option<MouthFillChargeDepthsV1>,
    turn_quality: Option<DeepTurnQualityV1>,
    level_of_hypoxia: Option<DeepHypoxiaV1>,
    level_of_exertion: Option<DeepExertionV1>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveReportFieldV1Request")]
pub struct DeepDiveReportFieldV1 {
    pub dives: Option<Vec<DeepDiveReportFieldsV1>>,

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
