use super::{
    deep_dive::DeepDiveReportFieldsV1,
    deep_general::{DepthSafetyV1, DepthVolumeV1, DisciplineMaxDepthV1, ExhaleDivesV1},
    dyn_general::{DistanceTravelledV1, LongestDynamicV1},
    dynamic::DynamicReportFieldsV1,
    fun_dive_general::FunDiveVolumeV1,
    gear_general::{FinsV1, MaskV1, NoseClipV1, WeightWornV1, WetsuitV1},
    general::{LocationV1, PersonalIncidentV1, SessionNameV1, StartTimeV1},
    general_environment::{
        AirTempV1, AlgaeV1, CurrentV1, EnvironmentEventV1, PollenV1, RainV1, VisibilityV1,
        WaterFeatureV1, WaterTempV1, WavesV1, WildlifeV1, WindV1,
    },
    general_mental_physical::{
        ComfortInGearV1, EaseOfEqualizationV1, GeneralFeelingV1, TirednessAfterV1,
        TirednessBeforeV1,
    },
    pre_session_general::{LastMealV1, QualityOfSleepV1, StimulationV1, StomachStatusV1},
    static_general::StaticVolumeV1,
    static_hold::StaticReportFieldsV1,
};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

// TODO: BIG QUESTION: Should inner fields be optional??

// REPORT
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "ReportV1Request")]
pub struct ReportV1 {
    // INDIVIDUAL APNEA HOLDS
    pub deep_dives: Option<Vec<DeepDiveReportFieldsV1>>,
    pub dynamic_dives: Option<Vec<DynamicReportFieldsV1>>,
    pub static_holds: Option<Vec<StaticReportFieldsV1>>,

    // DYNAMIC GENERAL
    distance_travelled: Option<DistanceTravelledV1>,
    longest_dynamic: Option<LongestDynamicV1>,

    // DEPTH GENERAL
    depth_volume: Option<DepthVolumeV1>,
    discipline_and_max_depth: Option<Vec<DisciplineMaxDepthV1>>,
    exhale_dives: Option<ExhaleDivesV1>,
    // TODO: Move to Session INfo
    depth_safety: Option<DepthSafetyV1>,

    // STATIC GENERAL
    static_volume: Option<StaticVolumeV1>,

    // FUN GENERAL
    fun_dive_volume: Option<FunDiveVolumeV1>,

    // GENERAL
    // SESSION INFO
    start_time: StartTimeV1,
    session_name: Option<SessionNameV1>,
    location: Option<LocationV1>,

    // PRE SESSION
    quality_of_sleep: Option<QualityOfSleepV1>,
    stimulation: Option<StimulationV1>,
    last_meal: Option<LastMealV1>,

    // GEAR
    weight_worn: Option<WeightWornV1>,
    wetsuit: Option<WetsuitV1>,
    fins: Option<FinsV1>,
    noseclip: Option<NoseClipV1>,
    mask: Option<MaskV1>,

    // MENTAL/PHYSICAL
    general_feeling: Option<GeneralFeelingV1>,
    ease_of_equalization: Option<EaseOfEqualizationV1>,
    tiredness_before: Option<TirednessBeforeV1>,
    tiredness_after: Option<TirednessAfterV1>,
    comfort_in_gear: Option<ComfortInGearV1>,
    // TODO This is actually stomach issues
    stomach_issues: Option<StomachStatusV1>,

    // ENVIRONMENT
    current: Option<CurrentV1>,
    visibility: Option<VisibilityV1>,
    waves: Option<WavesV1>,
    water_temp: Option<WaterTempV1>,
    air_temp: Option<AirTempV1>,
    rain: Option<RainV1>,
    wind: Option<WindV1>,
    algae: Option<AlgaeV1>,
    pollen: Option<PollenV1>,
    wildlife: Option<Vec<WildlifeV1>>,
    water_features: Option<Vec<WaterFeatureV1>>,
    environment_events: Option<Vec<EnvironmentEventV1>>,

    // INCIDENTS
    personal_incidents: Option<Vec<PersonalIncidentV1>>,
}

// TODO: Should I have an Incomplete_Report type?
impl From<StoredReportV1> for ReportV1 {
    fn from(value: StoredReportV1) -> Self {
        ReportV1 {
            // INDIVIDUAL APNEA HOLDS
            deep_dives: None,
            dynamic_dives: None,
            static_holds: None,

            // DYNAMIC GENERAL
            distance_travelled: value.distance_travelled,
            longest_dynamic: value.longest_dynamic,

            // DEPTH GENERAL
            depth_volume: value.depth_volume,
            discipline_and_max_depth: value.discipline_and_max_depth,
            exhale_dives: value.exhale_dives,

            // STATIC GENERAL
            static_volume: value.static_volume,

            // FUN GENERAL
            fun_dive_volume: value.fun_dive_volume,

            // GENERAL
            // SESSION INFO
            start_time: value.start_time,
            session_name: value.session_name,
            location: value.location,

            // PRE SESSION
            quality_of_sleep: value.quality_of_sleep,
            stimulation: value.stimulation,
            last_meal: value.last_meal,

            // GEAR
            weight_worn: value.weight_worn,
            wetsuit: value.wetsuit,
            fins: value.fins,
            noseclip: value.noseclip,
            mask: value.mask,
            depth_safety: value.depth_safety,

            // MENTAL/PHYSICAL
            general_feeling: value.general_feeling,
            ease_of_equalization: value.ease_of_equalization,
            tiredness_before: value.tiredness_before,
            tiredness_after: value.tiredness_after,
            comfort_in_gear: value.comfort_in_gear,
            stomach_issues: value.stomach_issues,

            // ENVIRONMENT
            current: value.current,
            visibility: value.visibility,
            waves: value.waves,
            water_temp: value.water_temp,
            air_temp: value.air_temp,
            rain: value.rain,
            wind: value.wind,
            algae: value.algae,
            pollen: value.pollen,
            wildlife: value.wildlife,
            water_features: value.water_features,
            environment_events: value.environment_events,

            // INCIDENTS
            personal_incidents: value.personal_incidents,
        }
    }
}

// TODO: Can you have included/not included struct types? Or is that just OOP?
#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct StoredReportV1 {
    // INDIVIDUAL
    // NO LONGER STORED ON THE REPORT OBJECT

    // DYNAMIC GENERAL
    distance_travelled: Option<DistanceTravelledV1>,
    longest_dynamic: Option<LongestDynamicV1>,

    // DEPTH GENERAL
    depth_volume: Option<DepthVolumeV1>,
    discipline_and_max_depth: Option<Vec<DisciplineMaxDepthV1>>,
    exhale_dives: Option<ExhaleDivesV1>,
    depth_safety: Option<DepthSafetyV1>,

    // STATIC GENERAL
    static_volume: Option<StaticVolumeV1>,

    // FUN GENERAL
    fun_dive_volume: Option<FunDiveVolumeV1>,

    // GENERAL
    // SESSION INFO
    start_time: StartTimeV1,
    session_name: Option<SessionNameV1>,
    location: Option<LocationV1>,

    // PRE SESSION
    quality_of_sleep: Option<QualityOfSleepV1>,
    stimulation: Option<StimulationV1>,
    last_meal: Option<LastMealV1>,

    // GEAR
    weight_worn: Option<WeightWornV1>,
    wetsuit: Option<WetsuitV1>,
    fins: Option<FinsV1>,
    noseclip: Option<NoseClipV1>,
    mask: Option<MaskV1>,

    // MENTAL/PHYSICAL
    general_feeling: Option<GeneralFeelingV1>,
    ease_of_equalization: Option<EaseOfEqualizationV1>,
    tiredness_before: Option<TirednessBeforeV1>,
    tiredness_after: Option<TirednessAfterV1>,
    comfort_in_gear: Option<ComfortInGearV1>,
    stomach_issues: Option<StomachStatusV1>,

    // ENVIRONMENT
    current: Option<CurrentV1>,
    visibility: Option<VisibilityV1>,
    waves: Option<WavesV1>,
    water_temp: Option<WaterTempV1>,
    air_temp: Option<AirTempV1>,
    rain: Option<RainV1>,
    wind: Option<WindV1>,
    algae: Option<AlgaeV1>,
    pollen: Option<PollenV1>,
    wildlife: Option<Vec<WildlifeV1>>,
    water_features: Option<Vec<WaterFeatureV1>>,
    environment_events: Option<Vec<EnvironmentEventV1>>,

    // INCIDENTS
    personal_incidents: Option<Vec<PersonalIncidentV1>>,
}

impl From<ReportV1> for StoredReportV1 {
    fn from(value: ReportV1) -> Self {
        StoredReportV1 {
            // NOTE: The UNIQUE APNEAS ARE NOT INCLUDED HERE

            // DYNAMIC GENERAL
            distance_travelled: value.distance_travelled,
            longest_dynamic: value.longest_dynamic,

            // DEPTH GENERAL
            depth_volume: value.depth_volume,
            discipline_and_max_depth: value.discipline_and_max_depth,
            exhale_dives: value.exhale_dives,
            depth_safety: value.depth_safety,

            // STATIC GENERAL
            static_volume: value.static_volume,

            // FUN GENERAL
            fun_dive_volume: value.fun_dive_volume,

            // GENERAL
            // SESSION INFO
            start_time: value.start_time,
            session_name: value.session_name,
            location: value.location,

            // PRE SESSION
            quality_of_sleep: value.quality_of_sleep,
            stimulation: value.stimulation,
            last_meal: value.last_meal,

            // GEAR
            weight_worn: value.weight_worn,
            wetsuit: value.wetsuit,
            fins: value.fins,
            noseclip: value.noseclip,
            mask: value.mask,

            // MENTAL/PHYSICAL
            general_feeling: value.general_feeling,
            ease_of_equalization: value.ease_of_equalization,
            tiredness_before: value.tiredness_before,
            tiredness_after: value.tiredness_after,
            comfort_in_gear: value.comfort_in_gear,
            stomach_issues: value.stomach_issues,

            // ENVIRONMENT
            current: value.current,
            visibility: value.visibility,
            waves: value.waves,
            water_temp: value.water_temp,
            air_temp: value.air_temp,
            rain: value.rain,
            wind: value.wind,
            algae: value.algae,
            pollen: value.pollen,
            wildlife: value.wildlife,
            water_features: value.water_features,
            environment_events: value.environment_events,

            // INCIDENTS
            personal_incidents: value.personal_incidents,
        }
    }
}
