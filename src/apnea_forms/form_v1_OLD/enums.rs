use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
)]
pub enum DisciplinesEnum {
    CWT,
    CWTPullingUp,
    CNF,
    FIM,
    FIMWithFins,
    DYN,
    DNF,
}

// TODO: rename to traumatic event
#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum InjuryEnumV1 {
    // Mental events
    PanicAttack,
    // Weather-related
    HeatStroke,
    SunBurn,
    Hypothermia,
    // Pressure-related
    MaskSqueeze,
    TracheaSqueeze,
    LungSqueeze,
    EarDrumPerforation,
    MiddleEarSqueeze,
    SinusSqueeze,
    // O2
    LMC,
    Hypoxia,
    NitrogenNarcosis,
}

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
)]
pub enum TemperatureEnum {
    F,
    C,
}

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
//     // CompletedFormName,
//     // InWater
//     // MaxDepth,
//     // MaxDepthWithDiscipline,
//     WarmUp,
//     // Injury,
//     // General
//     GeneralFeeling,
//     EqualizationEase,
//     // Health
//     Condition,
//     // Congestion,
//     // Environment
//     // Visibility,
//     // CurrentStrength,
//     // WindStrength,
//     // WaveStrength,
//     // Rain,
//     // AirTemp,
//     // WaterTemp,
// }
