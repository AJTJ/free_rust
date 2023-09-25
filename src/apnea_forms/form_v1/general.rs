use super::enums::{PersonalIncidentEnumV1, TemperatureEnumV1};
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
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

// Start Time

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StartTimeV1Request")]
pub struct StartTimeV1 {
    time: DateTime<Utc>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

// End Time

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// #[graphql(input_name = "EndTimeV1Request")]
// pub struct EndTimeV1 {
//     time: Option<DateTime<Utc>>,
//     // // defaults
//     // is_active: Option<bool>,
//     // field_order: Option<i32>,
// }

// INJURY

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "InjuryV1Request")]
pub struct PersonalIncidentV1 {
    value: Option<PersonalIncidentEnumV1>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

// LOCATION

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "LocationV1Request")]
pub struct LocationV1 {
    // TODO: Get GeoJSON somehow
    coordinates: Option<bool>,
    // TODO: figure out sharing of locations
    shared_location_id: Option<Uuid>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}
