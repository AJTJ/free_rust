use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::enums::{
    FinsTypeEnumV1, MealQualitiesEnumV1, StomachStatusEnumV1, WeightMeasurementEnumV1,
    WetSuitSizeTypeEnumV1,
};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "WeightWornV1Request")]
pub struct WeightWornV1 {
    weight_grams: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "WetsuitV1Request")]
pub struct WetsuitV1 {
    thickness_mm: Option<i32>,
    wetsuit_size_type: Option<WetSuitSizeTypeEnumV1>,
    // TODO include construction type
    hood: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FinsV1Request")]
pub struct FinsV1 {
    fins_type: Option<Vec<FinsTypeEnumV1>>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "NoseClipV1Request")]
pub struct NoseClipV1 {
    value: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "MaskV1Request")]
pub struct MaskV1 {
    value: Option<bool>,
}
