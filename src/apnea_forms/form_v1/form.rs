use super::{
    deep_dive::DeepDiveFormV1, dynamic::DynamicFormV1, enums::FormGroupTypes,
    static_hold::StaticFormV1,
};
use crate::{
    apnea_forms::{
        actions::{archive_form::archive_form, insert_form::insert_form},
        dto::form_dto::{Form, FormDetails},
        forms_interface::FormResponse,
    },
    utility::errors::BigError,
};
use async_graphql::{Context, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormFieldOptionsV1Request")]
pub struct FormFieldOptionsV1 {
    is_active: Option<bool>,
    group: FormGroupTypes,
    field_order: Option<i32>,
}

// Every report has a unique form, therfore `is_active` and `field_order` values ONLY need to exist in the form.
#[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
#[graphql(input_name = "FormV1Request")]
pub struct FormV1 {
    // INDIVIDUAL APNEA HOLDS
    deep_dives: DeepDiveFormV1,
    dynamic_dives: DynamicFormV1,
    static_holds: StaticFormV1,

    // DYNAMIC GENERAL
    distance_travelled: Option<FormFieldOptionsV1>,
    longest_dynamic: Option<FormFieldOptionsV1>,

    // DEPTH GENERAL
    depth_volume: Option<FormFieldOptionsV1>,
    discipline_and_max_depth: Option<FormFieldOptionsV1>,
    exhale_dives: Option<FormFieldOptionsV1>,
    depth_safety: Option<FormFieldOptionsV1>,

    // STATIC GENERAL
    static_volume: Option<FormFieldOptionsV1>,

    // FUN GENERAL
    fun_dive_volume: Option<FormFieldOptionsV1>,

    // GENERAL
    // SESSION INFO
    start_time: Option<FormFieldOptionsV1>,
    // end_time: Option<FormFieldOptionsV1>,
    session_name: Option<FormFieldOptionsV1>,
    location: Option<FormFieldOptionsV1>,

    // PRE SESSION
    quality_of_sleep: Option<FormFieldOptionsV1>,
    stimulation: Option<FormFieldOptionsV1>,
    last_meal: Option<FormFieldOptionsV1>,

    // GEAR
    weight_worn: Option<FormFieldOptionsV1>,
    wetsuit: Option<FormFieldOptionsV1>,
    fins: Option<FormFieldOptionsV1>,
    noseclip: Option<FormFieldOptionsV1>,
    mask: Option<FormFieldOptionsV1>,

    // MENTAL/PHYSICAL
    general_feeling: Option<FormFieldOptionsV1>,
    ease_of_equalization: Option<FormFieldOptionsV1>,
    tiredness_before: Option<FormFieldOptionsV1>,
    tiredness_after: Option<FormFieldOptionsV1>,
    comfort_in_gear: Option<FormFieldOptionsV1>,
    stomach_issues: Option<FormFieldOptionsV1>,

    // ENVIRONMENT
    current: Option<FormFieldOptionsV1>,
    visibility: Option<FormFieldOptionsV1>,
    waves: Option<FormFieldOptionsV1>,
    water_temp: Option<FormFieldOptionsV1>,
    air_temp: Option<FormFieldOptionsV1>,
    rain: Option<FormFieldOptionsV1>,
    wind: Option<FormFieldOptionsV1>,
    algae: Option<FormFieldOptionsV1>,
    pollen: Option<FormFieldOptionsV1>,
    wildlife: Option<FormFieldOptionsV1>,
    water_features: Option<FormFieldOptionsV1>,
    environment_events: Option<FormFieldOptionsV1>,

    // INCIDENTS
    personal_incidents: Option<FormFieldOptionsV1>,
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
