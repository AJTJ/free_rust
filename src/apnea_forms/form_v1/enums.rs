use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
)]
pub enum DisciplinesEnumV1 {
    CWTB,
    CWTMonofin,
    CWTPullingUp,
    CNF,
    FIM,
    FIMHeadUp,
    FIMWithFins,
    DYN,
    DNF,
    VariableWeight,
}

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
    // Other Physical
    PhysicalInjury,
    Exhaustion,
    Dehydration,
}

// Incidents covers all other incidents that aren't personal injuries
#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum DeepDiveIncidentsEnumV1 {
    BuddyInjury,
    BuoyOrPlatformIssues,
    BoatIssues,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum DynamicIncidentsEnumV1 {
    BuddyInjury,
    EquipmentIssues,
}

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
)]
pub enum TemperatureEnumV1 {
    F,
    C,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum TurnReasonsEnumV1 {
    // Mental
    PanicAttack,
    NotFeelingIt,
    // Physical
    Exhaustion,
    // 02/C02
    C02BuildUp,
    Hypoxia,
    // Injury,
    Injury,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum DeepDiveSensationsV1 {
    Burping,
    Hiccups,
    UrgeToCough,
    UrgeToDefecate,
    HeavyLegs,
    HeavyArms,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum StaticStoppingEnumV1 {
    UrgeToBreathe,
    Hypoxia,
    BlackOut,
    Discomfort,
    UncomfortableThoughts,
}
