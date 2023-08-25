use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use super::enums::DisciplinesEnumV1;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StaticVolumeV1Request")]
pub struct StaticVolumeV1 {
    breath_holds: i32,
}

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// #[graphql(input_name = "StaticVolumeV1Request")]
// pub struct StaticSafetyV1 {
//     buddy_experience: Option<i32>,
// }
