// use super::enums::{DisciplinesEnum, InjuryEnumV1, TemperatureEnum};
// use crate::{
//     apnea_forms::{
//         actions::{archive_form::archive_form, insert_form::insert_form},
//         dto::form_dto::{Form, FormDetails},
//         helpers::FormResponse,
//     },
//     utility::errors::BigError,
// };
// use async_graphql::{Context, InputObject, Interface, OneofObject, SimpleObject, Union};
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};

// use uuid::Uuid;

// // Start Time

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// #[graphql(input_name = "StartTimeV1Request")]
// struct StartTimeV1 {
//     time: DateTime<Utc>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }
// // End Time

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// #[graphql(input_name = "EndTimeV1Request")]
// struct EndTimeV1 {
//     time: Option<DateTime<Utc>>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // Session Name

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// #[graphql(input_name = "SessionNameV1Request")]
// struct SessionNameV1 {
//     name: Option<String>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // Weather

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "WeatherV1Request")]
// struct WeatherV1 {
//     wind: Option<i32>,
//     waves: Option<i32>,
//     current: Option<i32>,
//     rain: Option<i32>,
//     air_temperature: Option<i32>,
//     is_farenheit: Option<bool>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // Discipline and Max Depth
// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// #[graphql(input_name = "InnerDisciplineMaxDepthV1Request")]
// struct InnerDisciplineMaxDepthV1 {
//     discipline: Option<DisciplinesEnum>,
//     max_depth: i32,
// }

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// #[graphql(input_name = "DisciplineAndMaxDepthV1Request")]
// struct DisciplineAndMaxDepthV1 {
//     discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthV1>>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // MAX DEPTH

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "MaxDepthV1Request")]
// struct MaxDepthV1 {
//     max_depth: Option<i32>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // EASE OF EQUALIZATION

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "EaseOfEqualizationRequest")]
// struct EaseOfEqualizationV1 {
//     value: Option<i32>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // VISIBILITY

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "VisibilityV1Request")]
// struct VisibilityV1 {
//     value: Option<i32>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // GENERAL FEELING

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "GeneralFeelingV1Request")]
// struct GeneralFeelingV1 {
//     value: Option<i32>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // INJURY

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "InjuryV1Request")]
// struct InjuryV1 {
//     value: Option<InjuryEnumV1>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // WATER TEMPERATURE

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "WaterTempV1Request")]
// struct WaterTempV1 {
//     value: Option<i32>,
//     measurement: Option<TemperatureEnum>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // STATIC

// #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone, Copy)]
// #[graphql(input_name = "StaticV1Request")]
// struct StaticV1 {
//     value: Option<i32>,
//     // defaults
//     field_order: Option<i32>,
//     is_used: Option<bool>,
// }

// // FORMS

// // Dynamic apnea distance
// //

// #[derive(Serialize, Deserialize, Debug, SimpleObject, InputObject, Clone)]
// #[graphql(input_name = "FormV1Request")]
// pub struct FormV1 {
//     // Only non-optional field so far
//     start_time: StartTimeV1,
//     end_time: Option<EndTimeV1>,
//     session_name: Option<SessionNameV1>,
//     weather: Option<WeatherV1>,
//     discipline_and_max_depth: Option<DisciplineAndMaxDepthV1>,
//     max_depth: Option<MaxDepthV1>,
//     ease_of_equalization: Option<EaseOfEqualizationV1>,
//     visibility: Option<VisibilityV1>,
//     general_feeling: Option<GeneralFeelingV1>,
//     injury: Option<InjuryV1>,
//     water_temp: Option<WaterTempV1>,
//     static_apnea: Option<StaticV1>,
//     // things to solve with enums or web-services:
//     // diet - list of foods
//     // wildlife - list of potential animals seen
// }

// // EXPERIMENTS
// #[derive(OneofObject, Serialize, Deserialize, Debug, Clone)]
// enum RequestFieldsV1 {
//     SessionNameV1(SessionNameV1),
// }

// // EXPERIMENTS
// #[derive(Union, Serialize, Deserialize, Debug, Clone)]
// enum ResponseFieldV1 {
//     SessionNameV1(SessionNameV1),
// }

// // EXPERIMENTS
// #[derive(Interface, Serialize, Deserialize, Debug, Clone)]
// #[graphql(field(name = "field_order", type = "&Option<i32>"))]
// enum ResponseInterfaceV1 {
//     SessionNameV1(SessionNameV1),
// }

// // EXPERIMENTS
// pub struct NewFormV1(Vec<ResponseFieldV1>);

// // EXPERIMENTS
// impl NewFormV1 {}

// // Logic

// impl FormV1 {
//     pub async fn insert_form(
//         &self,
//         ctx: &Context<'_>,
//         form_details: FormDetails,
//         user_id: &Uuid,
//     ) -> Result<Option<Form>, BigError> {
//         // TODO: perform validation
//         let form = insert_form(ctx, form_details, FormResponse::V1(self.clone()), user_id).await?;
//         Ok(form)
//     }

//     pub async fn modify_form(
//         &self,
//         ctx: &Context<'_>,
//         previous_form_id: &Uuid,
//         form_details: FormDetails,
//         user_id: &Uuid,
//     ) -> Result<Option<Form>, BigError> {
//         archive_form(ctx, previous_form_id, user_id).await?;
//         let form = insert_form(ctx, form_details, FormResponse::V1(self.clone()), user_id).await?;
//         Ok(form)
//     }
// }
