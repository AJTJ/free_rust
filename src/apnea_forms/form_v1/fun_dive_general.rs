use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use super::enums::DisciplinesEnumV1;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FunDiveVolumeV1Request")]
pub struct FunDiveVolumeV1 {
    dives: Option<i32>,
    depth_range: Option<i32>,
}
