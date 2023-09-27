use super::enums::{EnvironmentEventsEnumV1, WaterFeaturesEnumV1};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

// CURRENT

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "CurrentV1Request")]
pub struct CurrentV1 {
    value: Option<i32>,
}

// WATER TEMPERATURE

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "WaterTempV1Request")]
pub struct WaterTempV1 {
    temp_celcius: i32,
}

// VISIBILITY

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "VisibilityV1Request")]
pub struct VisibilityV1 {
    value: Option<i32>,
}

// WAVES

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "WavesV1Request")]
pub struct WavesV1 {
    value: Option<i32>,
}

// AIR TEMP

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "AirTempV1Request")]
pub struct AirTempV1 {
    temp_celcius: i32,
}

// RAIN

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "RainV1Request")]
pub struct RainV1 {
    value: Option<i32>,
}

// WIND

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "WindV1Request")]
pub struct WindV1 {
    value: Option<i32>,
}

// ALGAE

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "AlgaeV1Request")]
pub struct AlgaeV1 {
    value: Option<i32>,
}

// POLLEN

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "PollenV1Request")]
pub struct PollenV1 {
    value: Option<i32>,
}

// TODO Update wildlife to API

// WILDLIFE

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "WildlifeV1Request")]
pub struct WildlifeV1 {
    value: Option<Vec<bool>>,
}

// WATER FEATURES

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "WaterFeatureV1Request")]
pub struct WaterFeatureV1 {
    feature: Option<WaterFeaturesEnumV1>,
    depth: Option<i32>,
    swim_through_length: Option<i32>,
}

// ENVIRONMENT EVENTS

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "EnvironmentEventV1Request")]
pub struct EnvironmentEventV1 {
    event: Option<EnvironmentEventsEnumV1>,
    // severity: Option<i32>,
}
