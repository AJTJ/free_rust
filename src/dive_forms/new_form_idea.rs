use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::using_new_form::FormTrait;

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
enum FishEnum {
    Big,
    Medium,
    Small,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
struct Fish {
    value: FishEnum,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
pub struct Form {
    fish: Option<Fish>,
}

impl FormTrait for Form {
    fn return_form(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
struct FishInput {
    value: FishEnum,
}

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
pub struct FormInput1_0_0 {
    fish: Option<FishInput>,
}

impl FormTrait for FormInput1_0_0 {
    fn return_form(&self) -> Self {
        self.clone()
    }
}