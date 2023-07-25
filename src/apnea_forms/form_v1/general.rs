use super::enums::{InjuryEnumV1, TemperatureEnumV1};
use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// GENERAL

// Session Name

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "SessionNameV1Request")]
pub struct SessionNameV1 {
    name: Option<String>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// End Time

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "EndTimeV1Request")]
pub struct EndTimeV1 {
    time: Option<DateTime<Utc>>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// EASE OF EQUALIZATION

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "EaseOfEqualizationRequest")]
pub struct EaseOfEqualizationV1 {
    value: Option<i32>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// VISIBILITY

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "VisibilityV1Request")]
pub struct VisibilityV1 {
    value: Option<i32>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// GENERAL FEELING

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "GeneralFeelingV1Request")]
pub struct GeneralFeelingV1 {
    value: Option<i32>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// INJURY

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "InjuryV1Request")]
pub struct InjuryV1 {
    value: Option<InjuryEnumV1>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// WATER TEMPERATURE

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "WaterTempV1Request")]
pub struct WaterTempV1 {
    value: Option<i32>,
    measurement: Option<TemperatureEnumV1>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}
// LOCATION

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "CoordinatesV1Request")]
struct CoordinatesV1 {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "LocationV1Request")]
pub struct LocationV1 {
    coordinates: Option<CoordinatesV1>,
    shared_location_id: Option<Uuid>,
    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}
