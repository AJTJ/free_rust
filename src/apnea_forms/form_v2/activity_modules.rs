use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use super::enums::DisciplinesEnumV2;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "InnerDisciplineMaxDepthV2Request")]
struct InnerDisciplineMaxDepthV2 {
    discipline: Option<DisciplinesEnumV2>,
    max_depth: i32,
}

// Discipline and Max Depth

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DisciplineAndMaxDepthV2Request")]
pub struct DisciplineAndMaxDepthV2 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthV2>>,
    // defaults
    field_order: Option<i32>,
}
