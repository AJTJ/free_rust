use std::str::FromStr;

use crate::dto::completed_form_dto::CompletedForm;
use crate::dto::completed_form_field_dto::CompletedFormField;
use crate::dto::form_dto::{Form, FormOutput};
use crate::dto::form_field_dto::FormField;
use crate::errors::{BigError, StrumParseSnafu};
use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use strum::IntoEnumIterator;
use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AllCustomEnums {}

#[derive(Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq)]
pub enum FieldValueTypes {
    Number,
    Enum,
    Timestamp,
    Interval,
    Text,
    EnumANDNumber,
}

#[derive(Serialize, Deserialize)]
pub struct EnumAndNumberStruct {
    number_value: String,
    enum_value: String,
    enum_type: String,
}

#[derive(Enum, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Debug, EnumString, Display)]
pub enum FieldNames {
    // InWater
    MaxDepth,
    MaxDepthWithDiscipline,
    WarmUp,
    Injury,
    // General
    GeneralFeeling,
    EqualizationEase,
    // Health
    Condition,
    Congestion,
    // Environment
    Visibility,
    CurrentStrength,
    WindStrength,
    WaveStrength,
    Rain,
    AirTemp,
    WaterTemp,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq)]
pub enum CategoryNames {
    General,
    Environment,
    InWater,
    Health,
    // pre dive
    Exertion,
    Sleep,
    Food,
    PreviousDay,
}

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq, EnumIter,
)]
pub enum Disciplines {
    CWT,
    CNF,
    FIM,
    DNF,
    STA,
}

#[derive(InputObject, Serialize, Deserialize, Clone, Debug)]
pub struct FSField {
    pub field_name: FieldNames,
    pub field_value: Option<String>,
    pub category_name: CategoryNames,
    pub field_value_type: FieldValueTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
pub struct FSFieldOutput {
    pub field_name: FieldNames,
    pub field_value: Option<String>,
    pub category_name: CategoryNames,
    pub field_value_type: FieldValueTypes,
}

impl FSFieldOutput {
    pub fn from_form_field(f: &FormField) -> Result<Self, BigError> {
        let cat_name = CategoryNames::from_str(&f.category_name).context(StrumParseSnafu)?;
        let field_name = FieldNames::from_str(&f.field_name).context(StrumParseSnafu)?;
        let field_value_type =
            FieldValueTypes::from_str(&f.field_value_type).context(StrumParseSnafu)?;
        Ok(FSFieldOutput {
            field_name,
            field_value: f.field_value.clone(),
            category_name: cat_name,
            field_value_type,
        })
    }

    pub fn from_completed_form_field(f: &CompletedFormField) -> Result<Self, BigError> {
        let cat_name = CategoryNames::from_str(&f.category_name).context(StrumParseSnafu)?;
        let field_name = FieldNames::from_str(&f.field_name).context(StrumParseSnafu)?;
        let field_value_type =
            FieldValueTypes::from_str(&f.field_value_type).context(StrumParseSnafu)?;
        Ok(FSFieldOutput {
            field_name,
            field_value: f.field_value.clone(),
            category_name: cat_name,
            field_value_type,
        })
    }
}

impl From<FSField> for FSFieldOutput {
    fn from(value: FSField) -> Self {
        FSFieldOutput {
            field_name: value.field_name,
            field_value: value.field_value,
            category_name: value.category_name,
            field_value_type: value.field_value_type,
        }
    }
}

impl From<FSFieldOutput> for FSField {
    fn from(value: FSFieldOutput) -> Self {
        FSField {
            field_name: value.field_name,
            field_value: value.field_value,
            category_name: value.category_name,
            field_value_type: value.field_value_type,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct FormStructure {
    pub form_template_version: Vec<i32>,
    pub form_id: Option<Uuid>,
    pub enums: Option<Vec<EnumLists>>,
    pub all_fields: Vec<FSField>,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct FormStructureOutput {
    pub form_template_version: Vec<i32>,
    pub form_id: Option<Uuid>,
    pub enums: Option<Vec<EnumListsOutput>>,
    pub all_fields: Vec<FSFieldOutput>,
}

impl From<FormStructure> for FormStructureOutput {
    fn from(value: FormStructure) -> Self {
        let my_enums = match value.enums {
            Some(e) => Some(
                e.into_iter()
                    .map(|e| EnumListsOutput::from(e))
                    .collect::<Vec<EnumListsOutput>>(),
            ),
            None => None,
        };

        let my_fields = value
            .all_fields
            .into_iter()
            .map(|f| FSFieldOutput::from(f))
            .collect();

        FormStructureOutput {
            form_template_version: value.form_template_version,
            form_id: value.form_id,
            enums: my_enums,
            all_fields: my_fields,
        }
    }
}

impl From<FormStructureOutput> for FormStructure {
    fn from(value: FormStructureOutput) -> Self {
        let my_enums = match value.enums {
            Some(e) => Some(
                e.into_iter()
                    .map(|e| EnumLists::from(e))
                    .collect::<Vec<EnumLists>>(),
            ),
            None => None,
        };

        let my_fields = value
            .all_fields
            .into_iter()
            .map(|f| FSField::from(f))
            .collect();

        FormStructure {
            form_template_version: value.form_template_version,
            form_id: value.form_id,
            enums: my_enums,
            all_fields: my_fields,
        }
    }
}

#[derive(InputObject, Serialize, Deserialize, Clone)]
pub struct EnumLists {
    field_name: FieldNames,
    enums: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct EnumListsOutput {
    field_name: FieldNames,
    enums: Vec<String>,
}

impl From<EnumLists> for EnumListsOutput {
    fn from(value: EnumLists) -> Self {
        EnumListsOutput {
            field_name: value.field_name,
            enums: value.enums,
        }
    }
}

impl From<EnumListsOutput> for EnumLists {
    fn from(value: EnumListsOutput) -> Self {
        EnumLists {
            field_name: value.field_name,
            enums: value.enums,
        }
    }
}

impl FormStructure {
    /*
    the user is creating or editin a form, and we replace it with our own preset values
       TODO: Versioning
       - if the user is updating an old form that uses an older form template, we need to take that into account.
    */
    pub fn validate_form(&self) -> Result<FormStructure, BigError> {
        let template = FormStructure::get_versioned_form_template(&self.form_template_version);

        let mut new_fields = vec![];

        for field in &self.all_fields {
            // check that there is a related template field
            let template_field = template
                .all_fields
                .iter()
                .find(|e| e.field_name == field.field_name);

            if let Some(template_field) = template_field {
                if template_field.category_name != field.category_name
                    || template_field.field_value_type != field.field_value_type
                {
                    return Err(BigError::FormFieldNotMatching);
                }

                if let Some(val) = &field.field_value {
                    let val_ok = match field.field_value_type {
                        FieldValueTypes::Number => val
                            .parse::<i32>()
                            .map_err(|e| BigError::ParseIntError { source: e })
                            .is_ok(),
                        FieldValueTypes::Enum => template
                            .enums
                            .as_ref()
                            .and_then(|e| {
                                e.iter()
                                    .find(|e| e.field_name == field.field_name)
                                    .and_then(|e| Some(e.enums.contains(&val)))
                            })
                            .is_some(),

                        FieldValueTypes::Timestamp => NaiveDateTime::from_str(&val)
                            .map_err(|e| BigError::ChronoParseError { source: e })
                            .is_ok(),
                        FieldValueTypes::Interval => val
                            .parse::<u64>()
                            .map_err(|e| BigError::ParseIntError { source: e })
                            .is_ok(),
                        FieldValueTypes::Text => true,
                        FieldValueTypes::EnumANDNumber => {
                            let struct_val = serde_json::from_str::<EnumAndNumberStruct>(&val)
                                .map_err(|e| BigError::SerdeParseError { source: e });

                            match struct_val {
                                Ok(struct_val) => {
                                    let enum_res = template
                                        .enums
                                        .as_ref()
                                        .and_then(|e| {
                                            e.iter()
                                                .find(|e| e.field_name == field.field_name)
                                                .and_then(|e| {
                                                    Some(e.enums.contains(&struct_val.enum_type))
                                                })
                                        })
                                        .is_some();

                                    let number_res = struct_val
                                        .number_value
                                        .parse::<i32>()
                                        .map_err(|e| BigError::ParseIntError { source: e })
                                        .is_ok();

                                    enum_res || number_res
                                }
                                Err(_) => false,
                            }
                        }
                    };
                    if !val_ok {
                        return Err(BigError::FormValueNotMatching);
                    }
                }

                // push the input form to the new fields, thus keeping the value if it is completed
                new_fields.push(field.clone())
            } else {
                // no related template field
                return Err(BigError::FormFieldNotMatching);
            }
        }

        // Does this make sense?
        Ok(FormStructure {
            form_id: self.form_id.clone(),
            enums: template.enums,
            form_template_version: template.form_template_version,
            all_fields: new_fields,
        })
    }

    // TODO: Probably get this from JSON/DOCUMENTATION files
    pub fn get_versioned_form_template(version: &Vec<i32>) -> FormStructure {
        // SHOULD DO A SEARCH HERE OF ALL THE FORMS
        let output = Self::get_latest_form_template();
        output.into()
    }
    // TODO: Probably get this from JSON/DOCUMENTATION files
    pub fn get_latest_form_template() -> FormStructureOutput {
        FormStructureOutput {
            form_template_version: vec![1, 0, 0],
            form_id: None,
            enums: Some(vec![EnumListsOutput {
                field_name: FieldNames::MaxDepthWithDiscipline,
                enums: Disciplines::iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
            }]),
            all_fields: vec![
                (FSFieldOutput {
                    field_value: None,
                    field_name: FieldNames::GeneralFeeling,
                    category_name: CategoryNames::General,
                    field_value_type: FieldValueTypes::Number,
                }),
                (FSFieldOutput {
                    field_value: None,
                    field_name: FieldNames::MaxDepthWithDiscipline,
                    category_name: CategoryNames::InWater,
                    field_value_type: FieldValueTypes::EnumANDNumber,
                }),
            ],
        }
    }

    pub fn construct_from_form(
        forms: Vec<Form>,
        fields: Vec<FormField>,
    ) -> Result<Vec<FormOutput>, BigError> {
        let form_structure_outputs = forms
            .iter()
            .map(|form| {
                let my_fields = fields
                    .iter()
                    .filter(|f| f.form_id == form.id)
                    .cloned()
                    .collect::<Vec<FormField>>();

                // TODO: This should get by the version number eventually
                let structure = FormStructure::get_latest_form_template();

                let version = form.template_version.iter().map(|n| n.unwrap()).collect();

                let all_fields = my_fields
                    .iter()
                    .map(|f| FSFieldOutput::from_form_field(f))
                    .collect::<Result<Vec<FSFieldOutput>, BigError>>()?;

                Ok(FormOutput {
                    form: form.clone(),
                    form_structure: FormStructureOutput {
                        form_template_version: version,
                        form_id: Some(form.id),
                        enums: structure.enums,
                        all_fields,
                    },
                })
            })
            .collect::<Result<Vec<FormOutput>, BigError>>();
        form_structure_outputs
    }

    pub fn construct_from_completed_form(
        forms: Vec<CompletedForm>,
        fields: Vec<CompletedFormField>,
    ) -> Result<Vec<(String, FormStructureOutput)>, BigError> {
        let form_structure_outputs = forms
            .iter()
            .map(|form| {
                let fields = fields
                    .iter()
                    .filter(|f| f.completed_form_id == form.id)
                    .cloned()
                    .collect::<Vec<CompletedFormField>>();

                // TODO: This should get by the version number eventually
                let structure = FormStructure::get_latest_form_template();

                let version = form.template_version.iter().map(|n| n.unwrap()).collect();

                let all_fields = fields
                    .iter()
                    .map(|f| FSFieldOutput::from_completed_form_field(f))
                    .collect::<Result<Vec<FSFieldOutput>, BigError>>()?;

                Ok((
                    form.created_at.to_string(),
                    FormStructureOutput {
                        form_template_version: version,
                        form_id: Some(form.id),
                        enums: structure.enums,
                        all_fields,
                    },
                ))
            })
            .collect::<Result<Vec<(String, FormStructureOutput)>, BigError>>();
        form_structure_outputs
    }
}

/*
WHAT IS HAPPENING
 - Send the user a filled out json object derived from the form_structure
 - Receive a list of input names
 - Create a new form from the input names, but derived from the form_structure
*/
#[cfg(test)]
mod tests {
    #[test]
    fn form_lifecycle() {
        use crate::dive_forms::form_helper::*;

        // user gets the latest form template fields and their values etc...
        // crucially this is everything they need to save and use forms
        let json_form = FormStructure::get_latest_form_template();

        // client returns a custom form
        let custom_form = FormStructure {
            form_template_version: vec![1, 0, 0],
            form_id: None,
            enums: None,
            all_fields: vec![FSField {
                field_value: None,
                field_name: FieldNames::GeneralFeeling,
                category_name: CategoryNames::General,
                field_value_type: FieldValueTypes::Number,
            }],
        };

        // get new form based on the enums from the client
        let validated_new_form = custom_form.validate_form();

        // they create a completed form with it
        let completed_form = FormStructure {
            form_template_version: vec![1, 0, 0],
            form_id: None,
            enums: None,
            all_fields: vec![FSField {
                field_value: Some("100".to_string()),
                field_name: FieldNames::GeneralFeeling,
                category_name: CategoryNames::General,
                field_value_type: FieldValueTypes::Number,
            }],
        };

        // this new form is validated

        let validated_completed_form = completed_form.validate_form().unwrap();

        // and is then stored in the database
    }
}
