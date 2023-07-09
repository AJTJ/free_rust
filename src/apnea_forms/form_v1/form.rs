use super::enums::{DisciplinesEnum, WildlifeEnumV1};
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
use async_graphql::{Context, Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
// use snafu::ResultExt;
// use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

// Report Name

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
struct SessionNameRequestV1 {
    name: Option<String>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
struct SessionNameResponseV1 {
    name: Option<String>,
    // defaults
    field_order: Option<i32>,
}

impl From<SessionNameRequestV1> for SessionNameResponseV1 {
    fn from(value: SessionNameRequestV1) -> Self {
        SessionNameResponseV1 {
            name: value.name,
            field_order: value.field_order,
        }
    }
}

// Wildlife

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct WildlifeRequestV1 {
    value: Option<WildlifeEnumV1>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct WildlifeResponseV1 {
    value: Option<WildlifeEnumV1>,
    // defaults
    field_order: Option<i32>,
}

impl From<WildlifeRequestV1> for WildlifeResponseV1 {
    fn from(value: WildlifeRequestV1) -> Self {
        WildlifeResponseV1 {
            value: value.value,
            field_order: value.field_order,
        }
    }
}

// Weather

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct WeatherRequestV1 {
    wind: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct WeatherResponseV1 {
    wind: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

impl From<WeatherRequestV1> for WeatherResponseV1 {
    fn from(value: WeatherRequestV1) -> Self {
        WeatherResponseV1 {
            wind: value.wind,
            field_order: value.field_order,
        }
    }
}

// Discipline and Max Depth
#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
struct InnerDisciplineMaxDepthRequestV1 {
    discipline: Option<DisciplinesEnum>,
    max_depth: i32,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
struct InnerDisciplineMaxDepthResponseV1 {
    discipline: Option<DisciplinesEnum>,
    max_depth: i32,
}

impl From<InnerDisciplineMaxDepthRequestV1> for InnerDisciplineMaxDepthResponseV1 {
    fn from(value: InnerDisciplineMaxDepthRequestV1) -> Self {
        InnerDisciplineMaxDepthResponseV1 {
            discipline: value.discipline,
            max_depth: value.max_depth,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
struct DisciplineAndMaxDepthRequestV1 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthRequestV1>>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
struct DisciplineAndMaxDepthResponseV1 {
    discipline_max_depth: Option<Vec<InnerDisciplineMaxDepthResponseV1>>,
    // defaults
    field_order: Option<i32>,
}

impl From<DisciplineAndMaxDepthRequestV1> for DisciplineAndMaxDepthResponseV1 {
    fn from(value: DisciplineAndMaxDepthRequestV1) -> Self {
        let discipline_max_depth = value
            .discipline_max_depth
            .and_then(|x| Some(x.into_iter().map(|l| l.into()).collect()));
        DisciplineAndMaxDepthResponseV1 {
            discipline_max_depth,
            field_order: value.field_order,
        }
    }
}

// MAX DEPTH

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct MaxDepthRequestV1 {
    max_depth: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct MaxDepthResponseV1 {
    max_depth: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

impl From<MaxDepthRequestV1> for MaxDepthResponseV1 {
    fn from(value: MaxDepthRequestV1) -> Self {
        MaxDepthResponseV1 {
            max_depth: value.max_depth,
            field_order: value.field_order,
        }
    }
}

// CONGESTION

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct CongestionRequestV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct CongestionResponseV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

impl From<CongestionRequestV1> for CongestionResponseV1 {
    fn from(value: CongestionRequestV1) -> Self {
        CongestionResponseV1 {
            value: value.value,
            field_order: value.field_order,
        }
    }
}

// VISIBILITY

#[derive(Serialize, Deserialize, Debug, InputObject, Clone, Copy)]
struct VisibilityRequestV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone, Copy)]
struct VisibilityResponseV1 {
    value: Option<i32>,
    // defaults
    field_order: Option<i32>,
}

impl From<VisibilityRequestV1> for VisibilityResponseV1 {
    fn from(value: VisibilityRequestV1) -> Self {
        VisibilityResponseV1 {
            value: value.value,
            field_order: value.field_order,
        }
    }
}

// FORMS

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
pub struct FormRequestV1 {
    session_name: Option<SessionNameRequestV1>,
    wildlife: Option<WildlifeRequestV1>,
    weather: Option<WeatherRequestV1>,
    discipline_and_max_depth: Option<DisciplineAndMaxDepthRequestV1>,
    max_depth: Option<MaxDepthRequestV1>,
    congestion: Option<CongestionRequestV1>,
    visibility: Option<VisibilityRequestV1>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct FormResponseV1 {
    session_name: Option<SessionNameResponseV1>,
    wildlife: Option<WildlifeResponseV1>,
    weather: Option<WeatherResponseV1>,
    discipline_and_max_depth: Option<DisciplineAndMaxDepthResponseV1>,
    max_depth: Option<MaxDepthResponseV1>,
    congestion: Option<CongestionResponseV1>,
    visibility: Option<VisibilityResponseV1>,
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
        report_details: ReportDetails,
    ) -> Result<Report, BigError> {
        // TODO: perform validation?
        let report = insert_report(
            ctx,
            session_id,
            report_details,
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
