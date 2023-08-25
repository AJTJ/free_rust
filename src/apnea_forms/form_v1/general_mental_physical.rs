use super::enums::{PersonalIncidentEnumV1, TemperatureEnumV1};
use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// GENERAL FEELING

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "GeneralFeelingV1Request")]
pub struct GeneralFeelingV1 {
    value: Option<i32>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

// EASE OF EQUALIZATION

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "EaseOfEqualizationV1Request")]
pub struct EaseOfEqualizationV1 {
    value: Option<i32>,
}

// Tiredness Before

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "TirednessBeforeV1Request")]
pub struct TirednessBeforeV1 {
    value: Option<i32>,
}

// Tiredness After

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "TirednessAfterV1Request")]
pub struct TirednessAfterV1 {
    value: Option<i32>,
}

// Comfort In Gear

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "ComfortInGearV1Request")]
pub struct ComfortInGearV1 {
    value: Option<i32>,
}
