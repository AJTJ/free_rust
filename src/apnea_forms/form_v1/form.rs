use async_graphql::{Context, Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

use crate::{
    apnea_forms::{
        actions::{insert_form::insert_form, insert_report::insert_report},
        dto::{form_dto::FormDetailsInput, report_dto::ReportDetailsInput},
        helpers::FormOutput,
    },
    utility::errors::{BigError, SerdeSerializeSnafu},
};

use super::enums::{DisciplinesEnum, WildlifeEnumV1};

// Report Name

#[derive(Serialize, Deserialize, InputObject, Clone)]
struct ReportNameInputV1 {
    name: String,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone)]
struct ReportNameOutputV1 {
    name: String,
    // defaults
    field_order: Option<i32>,
}

impl From<ReportNameInputV1> for ReportNameOutputV1 {
    fn from(value: ReportNameInputV1) -> Self {
        ReportNameOutputV1 {
            name: value.name,
            field_order: value.field_order,
        }
    }
}

// Discipline and Max Depth

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
struct DisciplineAndMaxDepthInputV1 {
    discipline: DisciplinesEnum,
    max_depth: i32,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
struct DisciplineAndMaxDepthOutputV1 {
    discipline: DisciplinesEnum,
    max_depth: i32,
    // defaults
    field_order: Option<i32>,
}

impl From<DisciplineAndMaxDepthInputV1> for DisciplineAndMaxDepthOutputV1 {
    fn from(value: DisciplineAndMaxDepthInputV1) -> Self {
        DisciplineAndMaxDepthOutputV1 {
            discipline: value.discipline,
            max_depth: value.max_depth,
            field_order: value.field_order,
        }
    }
}

// Wildlife

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
struct WildlifeInputV1 {
    value: WildlifeEnumV1,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
struct WildlifeOutputV1 {
    value: WildlifeEnumV1,
    // defaults
    field_order: Option<i32>,
}

impl From<WildlifeInputV1> for WildlifeOutputV1 {
    fn from(value: WildlifeInputV1) -> Self {
        WildlifeOutputV1 {
            value: value.value,
            field_order: value.field_order,
        }
    }
}

// Weather

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
struct WeatherInputV1 {
    wind: i32,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
struct WeatherOutputV1 {
    wind: i32,
    // defaults
    field_order: Option<i32>,
}

impl From<WeatherInputV1> for WeatherOutputV1 {
    fn from(value: WeatherInputV1) -> Self {
        WeatherOutputV1 {
            wind: value.wind,
            field_order: value.field_order,
        }
    }
}

// Forms

#[derive(Serialize, Deserialize, InputObject, Clone)]
pub struct FormInputV1 {
    report_name: Option<ReportNameInputV1>,
    wildlife: Option<WildlifeInputV1>,
    weather: Option<WeatherInputV1>,
    discipline_and_max_depth: Option<DisciplineAndMaxDepthInputV1>,
}

#[derive(Serialize, Deserialize, SimpleObject, Clone)]
pub struct FormOutputV1 {
    report_name: Option<ReportNameOutputV1>,
    wildlife: Option<WildlifeOutputV1>,
    weather: Option<WeatherOutputV1>,
    discipline_and_max_depth: Option<DisciplineAndMaxDepthOutputV1>,
}

impl From<FormInputV1> for FormOutputV1 {
    fn from(value: FormInputV1) -> Self {
        let report_name = value.report_name.and_then(|x| Some(x.into()));
        let wildlife = value.wildlife.and_then(|x| Some(x.into()));
        let weather = value.weather.and_then(|x| Some(x.into()));
        let discipline_and_max_depth = value.discipline_and_max_depth.and_then(|x| Some(x.into()));
        FormOutputV1 {
            report_name,
            wildlife,
            weather,
            discipline_and_max_depth,
        }
    }
}

// Logic

impl FormOutputV1 {
    pub async fn insert_form(
        &self,
        ctx: &Context<'_>,
        form_input: FormDetailsInput,
    ) -> Result<FormOutput, BigError> {
        // TODO: perform validation
        let form = insert_form(ctx, form_input, FormOutput::V1(self.clone())).await?;
        Ok(form.form_data)
    }

    pub async fn modify_form(
        &self,
        ctx: &Context<'_>,
        previous_form_id: Uuid,
    ) -> Result<FormOutput, BigError> {
        // TODO: get previous form, apply modifications/changes
        // TODO: update database
        unimplemented!()
    }

    pub async fn insert_report(
        &self,
        ctx: &Context<'_>,
        report_input: ReportDetailsInput,
    ) -> Result<FormOutput, BigError> {
        // TODO: perform validation?
        let report = insert_report(ctx, report_input, FormOutput::V1(self.clone())).await?;
        Ok(report.report_data)
    }

    pub async fn modify_report(
        &self,
        ctx: &Context<'_>,
        previous_report_id: Uuid,
    ) -> Result<FormOutput, BigError> {
        // TODO: get previous report, apply modifications/changes
        // TODO: update database
        unimplemented!()
    }
}
