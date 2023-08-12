use super::{
    activity_modules::{DisciplineAndMaxDepthV1, MaxDepthV1},
    deep_dive::{DeepDiveFormV1, DeepDiveReportFieldsV1},
    dynamic::{DynamicFormV1, DynamicReportFieldsV1},
    enums::{InjuryEnumV1, TemperatureEnumV1},
    general::{
        EaseOfEqualizationV1, EndTimeV1, GeneralFeelingV1, InjuryV1, LocationV1, SessionNameV1,
        VisibilityV1, WaterTempV1,
    },
    static_activity::{StaticFormV1, StaticReportFieldsV1},
};
use crate::{
    apnea_forms::{
        actions::{archive_form::archive_form, insert_form::insert_form},
        dto::form_dto::{Form, FormDetails},
        forms_interface::FormResponse,
    },
    utility::errors::BigError,
};
use async_graphql::{Context, InputObject, OneofObject, SimpleObject, Union};
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    pg::{Pg, PgValue},
    sql_types,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "StartTimeV1Request")]
struct StartTimeV1 {
    time: DateTime<Utc>,
    // // defaults
    // is_active: Option<bool>,
    // field_order: Option<i32>,
}

// REPORT

// This will be the report that is received and sent to the client
// But it is NOT how it is stored in the database.
// It makes sense to provide a single report and a single form for the client. Let's make the client code easy!
// It provides a lot of simplicity if they are the same thing
// WHERE I store the report data is entirely a matter of database efficiency,
// ... and has NOTHING to do with the client
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "ReportV1Request")]
pub struct ReportV1 {
    // INDIVIDUAL
    pub deep_dives: Option<Vec<DeepDiveReportFieldsV1>>,
    pub dynamic_dives: Option<Vec<DynamicReportFieldsV1>>,
    pub static_holds: Option<Vec<StaticReportFieldsV1>>,
    // ACTIVITY-BASED
    discipline_and_max_depth: Option<DisciplineAndMaxDepthV1>,
    max_depth: Option<MaxDepthV1>,
    // GENERAL
    start_time: StartTimeV1,
    session_name: Option<SessionNameV1>,
    end_time: Option<EndTimeV1>,
    ease_of_equalization: Option<EaseOfEqualizationV1>,
    visibility: Option<VisibilityV1>,
    general_feeling: Option<GeneralFeelingV1>,
    injury: Option<InjuryV1>,
    water_temp: Option<WaterTempV1>,
    location: Option<LocationV1>,
    // REPORT SPECIFIC
}

impl From<StoredReportV1> for ReportV1 {
    fn from(value: StoredReportV1) -> Self {
        ReportV1 {
            deep_dives: None,
            dynamic_dives: None,
            static_holds: None,
            discipline_and_max_depth: value.discipline_and_max_depth,
            max_depth: value.max_depth,
            start_time: value.start_time,
            session_name: value.session_name,
            end_time: value.end_time,
            ease_of_equalization: value.ease_of_equalization,
            visibility: value.visibility,
            general_feeling: value.general_feeling,
            injury: value.injury,
            water_temp: value.water_temp,
            location: value.location,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct StoredReportV1 {
    // INDIVIDUAL
    // NO LONGER STORED ON THE REPORT OBJECT

    // ACTIVITY-BASED
    discipline_and_max_depth: Option<DisciplineAndMaxDepthV1>,
    max_depth: Option<MaxDepthV1>,
    // GENERAL
    start_time: StartTimeV1,
    session_name: Option<SessionNameV1>,
    end_time: Option<EndTimeV1>,
    ease_of_equalization: Option<EaseOfEqualizationV1>,
    visibility: Option<VisibilityV1>,
    general_feeling: Option<GeneralFeelingV1>,
    injury: Option<InjuryV1>,
    water_temp: Option<WaterTempV1>,
    location: Option<LocationV1>,
}

impl From<ReportV1> for StoredReportV1 {
    fn from(value: ReportV1) -> Self {
        StoredReportV1 {
            // NOTE: The UNIQUE APNEAS ARE NOT INCLUDED HERE
            discipline_and_max_depth: value.discipline_and_max_depth,
            max_depth: value.max_depth,
            start_time: value.start_time,
            session_name: value.session_name,
            end_time: value.end_time,
            ease_of_equalization: value.ease_of_equalization,
            visibility: value.visibility,
            general_feeling: value.general_feeling,
            injury: value.injury,
            water_temp: value.water_temp,
            location: value.location,
        }
    }
}

// FORM

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormFieldOptionsV1Request")]
pub struct FormFieldOptionsV1 {
    is_active: Option<bool>,
    field_order: Option<i32>,
}

// NOTE: Every report will ALWAYS have a unique form. If the user updates the order or included fields of a report, then we create a new report and a new form in the backend.
// Therefore, I can assure myself that all `is_active` and `field_order` values ONLY need to exist in the form.
// Is this true?
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormV1Request")]
pub struct FormV1 {
    // INDIVIDUAL
    deep_dives: DeepDiveFormV1,
    dynamic_dives: DynamicFormV1,
    static_holds: StaticFormV1,
    // ACTIVITY-BASED
    // Deep Diving
    discipline_and_max_depth: Option<FormFieldOptionsV1>,
    max_depth: Option<FormFieldOptionsV1>,
    // GENERAL
    // NOTE: start_time is required in a report, even though it is optional here
    start_time: Option<FormFieldOptionsV1>,
    session_name: Option<FormFieldOptionsV1>,
    end_time: Option<FormFieldOptionsV1>,
    ease_of_equalization: Option<FormFieldOptionsV1>,
    visibility: Option<FormFieldOptionsV1>,
    general_feeling: Option<FormFieldOptionsV1>,
    injury: Option<FormFieldOptionsV1>,
    water_temp: Option<FormFieldOptionsV1>,
    location: Option<FormFieldOptionsV1>,
    // FORM SPECIFIC
}

impl FormV1 {
    pub async fn insert_form(
        &self,
        ctx: &Context<'_>,
        form_details: FormDetails,
        user_id: &Uuid,
    ) -> Result<Option<Form>, BigError> {
        // TODO: perform validation
        let form = insert_form(ctx, form_details, FormResponse::V1(self.clone()), user_id).await?;
        Ok(form)
    }

    pub async fn modify_form(
        &self,
        ctx: &Context<'_>,
        previous_form_id: &Uuid,
        form_details: FormDetails,
        user_id: &Uuid,
    ) -> Result<Option<Form>, BigError> {
        archive_form(ctx, previous_form_id, user_id).await?;
        let form = insert_form(ctx, form_details, FormResponse::V1(self.clone()), user_id).await?;
        Ok(form)
    }
}

// TODO: write a simple test to ensure that every field in a form is in a report and vice versa?
