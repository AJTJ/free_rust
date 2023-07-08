use async_graphql::{Context, Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

use crate::{
    apnea_forms::{
        actions::{insert_form::insert_form, insert_report::insert_report},
        dto::{
            form_dto::{Form, FormDetails},
            report_dto::{Report, ReportDetails},
        },
        helpers::FormResponse,
    },
    utility::errors::BigError,
};

use super::enums::{DisciplinesEnum, WildlifeEnumV1};

// Report Name

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
struct SessionNameInputV1 {
    name: Option<String>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
struct SessionNameOutputV1 {
    name: Option<String>,
    // defaults
    field_order: Option<i32>,
}

impl From<SessionNameInputV1> for SessionNameOutputV1 {
    fn from(value: SessionNameInputV1) -> Self {
        SessionNameOutputV1 {
            name: value.name,
            field_order: value.field_order,
        }
    }
}

// Wildlife

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct WildlifeInputV1 {
    value: Option<WildlifeEnumV1>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct WildlifeOutputV1 {
    value: Option<WildlifeEnumV1>,
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

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct WeatherInputV1 {
    wind: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct WeatherOutputV1 {
    wind: Option<i32>,
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

// Discipline and Max Depth
#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
struct InnerDisciplineMaxDepthInputV1 {
    discipline: Option<DisciplinesEnum>,
    max_depth: i32,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
struct InnerDisciplineMaxDepthOutputV1 {
    discipline: Option<DisciplinesEnum>,
    max_depth: i32,
}

impl From<InnerDisciplineMaxDepthInputV1> for InnerDisciplineMaxDepthOutputV1 {
    fn from(value: InnerDisciplineMaxDepthInputV1) -> Self {
        InnerDisciplineMaxDepthOutputV1 {
            discipline: value.discipline,
            max_depth: value.max_depth,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
struct DisciplineAndMaxDepthInputV1 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthInputV1>>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
struct DisciplineAndMaxDepthOutputV1 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthOutputV1>>,
    // defaults
    field_order: Option<i32>,
}

impl From<DisciplineAndMaxDepthInputV1> for DisciplineAndMaxDepthOutputV1 {
    fn from(value: DisciplineAndMaxDepthInputV1) -> Self {
        let discipline_max_depth = value
            .discipline_max_depth
            .and_then(|x| Some(x.into_iter().map(|l| l.into()).collect()));
        DisciplineAndMaxDepthOutputV1 {
            discipline_max_depth,
            field_order: value.field_order,
        }
    }
}

// MAX DEPTH

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct MaxDepthInputV1 {
    max_depth: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct MaxDepthOutputV1 {
    max_depth: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

impl From<MaxDepthInputV1> for MaxDepthOutputV1 {
    fn from(value: MaxDepthInputV1) -> Self {
        MaxDepthOutputV1 {
            max_depth: value.max_depth,
            field_order: value.field_order,
        }
    }
}

// CONGESTION

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct CongestionInputV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct CongestionOutputV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

impl From<CongestionInputV1> for CongestionOutputV1 {
    fn from(value: CongestionInputV1) -> Self {
        CongestionOutputV1 {
            value: value.value,
            field_order: value.field_order,
        }
    }
}

// VISIBILITY

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct VisibilityInputV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct VisibilityOutputV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

impl From<VisibilityInputV1> for VisibilityOutputV1 {
    fn from(value: VisibilityInputV1) -> Self {
        VisibilityOutputV1 {
            value: value.value,
            field_order: value.field_order,
        }
    }
}

// FORMS

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
pub struct FormRequestV1 {
    session_name: Option<SessionNameInputV1>,
    wildlife: Option<WildlifeInputV1>,
    weather: Option<WeatherInputV1>,
    discipline_and_max_depth: Option<DisciplineAndMaxDepthInputV1>,
    max_depth: Option<MaxDepthInputV1>,
    congestion: Option<CongestionInputV1>,
    visibility: Option<VisibilityInputV1>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct FormResponseV1 {
    session_name: Option<SessionNameOutputV1>,
    wildlife: Option<WildlifeOutputV1>,
    weather: Option<WeatherOutputV1>,
    discipline_and_max_depth: Option<DisciplineAndMaxDepthOutputV1>,
    max_depth: Option<MaxDepthOutputV1>,
    congestion: Option<CongestionOutputV1>,
    visibility: Option<VisibilityOutputV1>,
}

impl From<FormRequestV1> for FormResponseV1 {
    fn from(value: FormRequestV1) -> Self {
        let session_name = value.session_name.and_then(|x| Some(x.into()));
        let wildlife = value.wildlife.and_then(|x| Some(x.into()));
        let weather = value.weather.and_then(|x| Some(x.into()));
        let discipline_and_max_depth = value.discipline_and_max_depth.and_then(|x| Some(x.into()));
        let max_depth = value.max_depth.and_then(|x| Some(x.into()));
        let congestion = value.congestion.and_then(|x| Some(x.into()));
        let visibility = value.visibility.and_then(|x| Some(x.into()));
        FormResponseV1 {
            session_name,
            wildlife,
            weather,
            discipline_and_max_depth,
            max_depth,
            congestion,
            visibility,
        }
    }
}

// Logic

impl FormResponseV1 {
    pub async fn insert_form(
        &self,
        ctx: &Context<'_>,
        form_request: FormDetails,
    ) -> Result<Form, BigError> {
        // TODO: perform validation
        let form = insert_form(ctx, form_request, FormResponse::V1(self.clone())).await?;
        Ok(form)
    }

    pub async fn modify_form(
        &self,
        ctx: &Context<'_>,
        previous_form_id: Uuid,
    ) -> Result<Form, BigError> {
        // TODO: get previous form, apply modifications/changes
        // TODO: update database
        unimplemented!()
    }

    pub async fn insert_report(
        &self,
        ctx: &Context<'_>,
        session_id: &Uuid,
        report_input: ReportDetails,
    ) -> Result<Report, BigError> {
        // TODO: perform validation?
        let report = insert_report(
            ctx,
            session_id,
            report_input,
            FormResponse::V1(self.clone()),
        )
        .await?;
        Ok(report)
    }

    pub async fn modify_report(
        &self,
        ctx: &Context<'_>,
        previous_report_id: Uuid,
    ) -> Result<Report, BigError> {
        // TODO: get previous report, apply modifications/changes
        // TODO: update database
        unimplemented!()
    }
}
