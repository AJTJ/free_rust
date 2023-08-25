use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use super::enums::{DepthSafetySetupEnumV1, DisciplinesEnumV1, ExhaleDivesEnumV1};

// DEPTH VOLUME

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "DepthVolumeV1Request")]
pub struct DepthVolumeV1 {
    dives: i32,
}

// DISCIPLINE AND MAX DEPTH

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "DisciplineMaxDepthV1Request")]
pub struct DisciplineMaxDepthV1 {
    discipline: Option<DisciplinesEnumV1>,
    max_depth: i32,
}

// EXHALE DIVES

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "ExhaleDivesDepthRangeV1Request")]
struct ExhaleDivesDepthRangeV1 {
    low: i32,
    high: i32,
}

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "ExhaleDivesV1Request")]
pub struct ExhaleDivesV1 {
    dives: Option<i32>,
    exhale_quantity: ExhaleDivesEnumV1,
    depth_range: Option<ExhaleDivesDepthRangeV1>,
}

// SAFETY SETUP

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
#[graphql(input_name = "DepthSafetyV1Request")]
pub struct DepthSafetyV1 {
    setup: DepthSafetySetupEnumV1,
    safety_experience: Option<i32>,
}
