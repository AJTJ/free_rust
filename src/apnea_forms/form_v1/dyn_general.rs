use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use super::enums::DisciplinesEnumV1;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DistanceTravelledV1Request")]
pub struct DistanceTravelledV1 {
    value: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "LongestDynamicV1Request")]
pub struct LongestDynamicV1 {
    value: i32,
    discipline: Option<DisciplinesEnumV1>,
}
