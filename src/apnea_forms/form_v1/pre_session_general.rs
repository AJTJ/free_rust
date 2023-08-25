use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::enums::{MealQualitiesEnumV1, StomachStatusEnumV1};

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "QualityOfSleepV1Request")]
pub struct QualityOfSleepV1 {
    value: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StimulationV1Request")]
pub struct StimulationV1 {
    value: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "LastMealV1Request")]
pub struct LastMealV1 {
    time: Option<DateTime<Utc>>,
    heavyness: Option<i32>,
    meal_qualities: Option<Vec<MealQualitiesEnumV1>>,
}

// TODO: This could be an enum?
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StomachStatusV1Request")]
pub struct StomachStatusV1 {
    status: StomachStatusEnumV1,
}
