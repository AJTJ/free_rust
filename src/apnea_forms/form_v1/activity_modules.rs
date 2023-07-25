use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use super::enums::DisciplinesEnumV1;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "InnerDisciplineMaxDepthV1Request")]
struct InnerDisciplineMaxDepthV1 {
    discipline: Option<DisciplinesEnumV1>,
    max_depth: i32,
}

// Discipline and Max Depth

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DisciplineAndMaxDepthV1Request")]
pub struct DisciplineAndMaxDepthV1 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthV1>>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// // MAX DEPTH

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "MaxDepthV1Request")]
pub struct MaxDepthV1 {
    max_depth: Option<i32>,

    // defaults
    is_active: Option<bool>,
    field_order: Option<i32>,
}
