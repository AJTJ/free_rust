use super::enums::{DisciplinesEnum, InjuryEnumV2, TemperatureEnum};
use crate::{
    apnea_forms::{
        actions::{archive_form::archive_form, insert_form::insert_form},
        dto::form_dto::{Form, FormDetails},
        form_V2::enums::DisciplinesEnum,
        helpers::FormResponse,
    },
    utility::errors::BigError,
};
use async_graphql::{Context, InputObject, Interface, OneofObject, SimpleObject, Union};
use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

// NOTES: there is a difference between the form and the report
// making them the same thing isn't necessarily simple

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

// Discipline and Max Depth
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "InnerDisciplineMaxDepthV2Request")]
struct InnerDisciplineMaxDepthV2 {
    discipline: Option<DisciplinesEnum>,
    max_depth: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DisciplineAndMaxDepthV2Request")]
struct DisciplineAndMaxDepthV2 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthV2>>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormFieldOptionsV2Request")]
struct FormFieldOptionsV2 {
    field_order: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveReportV2Request")]
struct DeepDiveReportFieldsV2 {
    discipline: Option<DisciplinesEnum>,
    goal_depth: Option<i32>,
    achieved_depth: Option<i32>,
    dive_time: Option<NaiveTime>,
    early_turn_depth: Option<i32>,
    reason_for_turning: Option<TurnReasonsEnum>,
    general_feeling: Option<DiveFeelingsEnum>,
    specific_sensations: Option<DiveSensationsEnum>,
    thoughts: Option<ThoughtsEnum>,
    incidents: Option<IncidentsEnum>,
    mouth_fill_depth: Option<i32>,
    mouth_fill_charge_depth: Option<i32>,
    turn_quality: Option<i32>,
    level_of_hypoxia: Option<i32>,
    level_of_exertion: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
struct DeepDiveReportV2 {
    dives: Vec<DeepDiveReportFieldsV2>,
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DeepDiveFormV2Request")]
struct DeepDiveFormV2 {
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
    mouth_fill_charge_depth: Option<FormFieldOptionsV2>,
    turn_quality: Option<FormFieldOptionsV2>,
    level_of_hypoxia: Option<FormFieldOptionsV2>,
    level_of_exertion: Option<FormFieldOptionsV2>,
}

struct FormV2 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveFormV2>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<FormFieldOptionsV2>,
    // GENERAL
    end_time: Option<FormFieldOptionsV2>,
    // FORM SPECIFIC
    form_name: String,
}

struct ReportV2 {
    // INDIVIDUAL
    deep_dives: Option<DeepDiveReportV2>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<DisciplineAndMaxDepthV2>,
    // GENERAL
    end_time: Option<EndTimeV2>,
    // REPORT SPECIFIC
    start_time: StartTimeV2,
}
