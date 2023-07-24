use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
)]
pub enum DisciplinesEnumV2 {
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
pub enum InjuryEnumV2 {
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
pub enum DeepDiveIncidentsEnumV2 {
    BuddyInjury,
    BuoyOrPlatformIssues,
    BoatIssues,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum DynamicIncidentsEnumV2 {
    BuddyInjury,
    EquipmentIssues,
}

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
)]
pub enum TemperatureEnumV2 {
    F,
    C,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum TurnReasonsEnumV2 {
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
pub enum DeepDiveSensationsV2 {
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
pub enum StaticStoppingEnumV2 {
    UrgeToBreathe,
    Hypoxia,
    BlackOut,
    Discomfort,
    UncomfortableThoughts,
}
