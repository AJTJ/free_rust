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
pub enum PersonalIncidentEnumV1 {
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
    ConflictAmongstPeople,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]

pub enum DynIncidentsEnumV1 {
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
    Anxiety,
    NotFeelingIt,
    // Physical
    Exhaustion,
    // 02/C02
    C02BuildUp,
    Hypoxia,
    // Injury,
    Injury,
    // Other
    Other,
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

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum ExhaleDivesEnumV1 {
    FRC,
    RV,
    LessThanFRC,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum WaterFeaturesEnumV1 {
    SwimThrough,
    ShipWreck,
    Vehicle,
    Plane,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum EnvironmentEventsEnumV1 {
    Storm,
    Hurricane,
    Tsunami,
    Tornado,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum MealQualitiesEnumV1 {
    Fruit,
    Oily,
    Fat,
    Carb,
    Protein,
    Acidic,
    Lactose,
    Sweetened,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum StomachStatusEnumV1 {
    AllGood,
    Full,
    VeryFull,
    BowelsUpset,
    SomePain,
    Pain,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum WeightMeasurementEnumV1 {
    Pounds,
    Kilograms,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum WetSuitSizeTypeEnumV1 {
    ThreeQuarter,
    Full,
}
#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum FinsTypeEnumV1 {
    BiFins,
    Monofin,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum DepthSafetySetupEnumV1 {
    Platform,
    MooredBuoy,
    DriftBuoy,
    NoLine,
}

#[derive(
    Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq, EnumIter,
)]
pub enum FormGroupTypes {
    // Individual apneas
    Dynamic,
    DeepDive,
    Static,

    // Activity General
    DynamicGeneral,
    DepthGeneral,
    StaticGeneral,
    FunGeneral,

    // Other General
    SessionInfo,
    PreSession,
    Gear,
    MentalPhysical,
    Environment,
    Incidents,
}
