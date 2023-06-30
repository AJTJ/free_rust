use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::form_trait::FormTrait;

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
enum WildlifeEnumV1 {
    Big,
    Medium,
    Small,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
struct WildlifeV1 {
    value: WildlifeEnumV1,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
struct WeatherV1 {
    wind: i32,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
pub struct FormOutputV1 {
    fish: Option<WildlifeV1>,
    weather: Option<WeatherV1>,
}

impl FormTrait for FormOutputV1 {
    fn return_form(&self) -> Self {
        self.clone()
    }
    fn validate_form(&self) -> Self {
        self.clone()
    }
    fn get_template() -> Self {
        Self {
            fish: todo!(),
            weather: todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
struct WildlifeInputV1 {
    value: WildlifeEnumV1,
}

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
struct WeatherInputV1 {
    wind: i32,
}

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
pub struct FormInputV1 {
    fish: Option<WildlifeInputV1>,
    weather: Option<WeatherInputV1>,
}

impl FormTrait for FormInputV1 {
    fn return_form(&self) -> Self {
        self.clone()
    }
    fn validate_form(&self) -> Self {
        self.clone()
    }
    fn get_template() -> Self {
        Self {
            fish: todo!(),
            weather: todo!(),
        }
    }
}

// #[derive(
//     Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
// )]
// pub enum FieldValueTypes {
//     Number,
//     Timestamp,
//     Interval,
//     Text,
// }

// #[derive(
//     Enum, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Debug, EnumString, Display, EnumIter,
// )]
// pub enum FieldNames {
//     // FormRelated
//     CompletedFormName,
//     // InWater
//     MaxDepth,
//     MaxDepthWithDiscipline,
//     WarmUp,
//     Injury,
//     // General
//     GeneralFeeling,
//     EqualizationEase,
//     // Health
//     Condition,
//     Congestion,
//     // Environment
//     Visibility,
//     CurrentStrength,
//     WindStrength,
//     WaveStrength,
//     Rain,
//     AirTemp,
//     WaterTemp,
// }
// #[derive(
//     Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
// )]
// pub enum CategoryNames {
//     FormRelated,
//     General,
//     Environment,
//     InWater,
//     Health,
//     // pre dive
//     Exertion,
//     Sleep,
//     Food,
//     PreviousDay,
// }

// #[derive(
//     Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
// )]
// pub enum DisciplinesEnum {
//     CWT,
//     CNF,
//     FIM,
//     DNF,
//     STA,
// }

// #[derive(
//     Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
// )]
// pub enum AllEnums {
//     DisciplinesEnum,
// }
