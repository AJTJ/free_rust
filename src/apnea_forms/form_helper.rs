use crate::dto::completed_form_dto::CompletedForm;
use crate::dto::completed_form_field_dto::CompletedFormField;
use crate::dto::form_dto::{Form, FormOutput};
use crate::dto::form_field_dto::FormField;
use crate::errors::BigError;
use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(InputObject, Serialize, Deserialize, Clone, Debug)]
pub struct FSField {
    pub field_order: Option<i32>,
    pub field_name: String,
    pub field_value: Option<Vec<String>>,
    pub category_name: String,
    pub field_value_type: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
pub struct FSFieldOutput {
    pub field_order: Option<i32>,
    pub field_name: String,
    pub field_value: Option<Vec<String>>,
    pub category_name: String,
    pub field_value_type: Vec<String>,
}

impl FSFieldOutput {
    pub fn from_form_field(f: &FormField) -> Result<Self, BigError> {
        // let cat_name = CategoryNames::from_str(&f.category_name).context(StrumParseSnafu)?;
        // let field_name = FieldNames::from_str(&f.field_name).context(StrumParseSnafu)?;

        // let field_value_type =
        //     FieldValueTypes::from_str(&f.field_value_type).context(StrumParseSnafu)?;

        let my_field_value = &f
            .field_value
            .clone()
            .map(|v| {
                v.into_iter()
                    .map(|maybe_str| {
                        if let Some(x) = maybe_str {
                            Ok(x)
                        } else {
                            Err(BigError::DieselIncorrectDBValues)
                        }
                    })
                    .collect::<Result<Vec<String>, BigError>>()
            })
            .transpose()?;

        let my_field_value_types = &f
            .field_value_type
            .clone()
            .into_iter()
            .map(|maybe_str| {
                if let Some(x) = maybe_str {
                    // FieldValueTypes::from_str(&x).context(StrumParseSnafu)
                    Ok(x)
                } else {
                    Err(BigError::DieselIncorrectDBValues)
                }
            })
            .collect::<Result<Vec<String>, BigError>>()?;

        Ok(FSFieldOutput {
            field_order: f.field_order,
            field_name: f.field_name.clone(),
            field_value: my_field_value.clone(),
            category_name: f.category_name.clone(),
            field_value_type: my_field_value_types.clone(),
        })
    }

    pub fn from_completed_form_field(f: &CompletedFormField) -> Result<Self, BigError> {
        // let cat_name = CategoryNames::from_str(&f.category_name).context(StrumParseSnafu)?;
        // let field_name = FieldNames::from_str(&f.field_name).context(StrumParseSnafu)?;
        // let field_value_type =
        //     FieldValueTypes::from_str(&f.field_value_type).context(StrumParseSnafu)?;

        let my_field_value = &f
            .field_value
            .clone()
            .map(|v| {
                v.into_iter()
                    .map(|maybe_str| {
                        if let Some(x) = maybe_str {
                            Ok(x)
                        } else {
                            Err(BigError::DieselIncorrectDBValues)
                        }
                    })
                    .collect::<Result<Vec<String>, BigError>>()
            })
            .transpose()?;

        let my_field_value_types = &f
            .field_value_type
            .clone()
            .into_iter()
            .map(|maybe_str| {
                if let Some(x) = maybe_str {
                    // FieldValueTypes::from_str(&x).context(StrumParseSnafu)
                    Ok(x)
                } else {
                    Err(BigError::DieselIncorrectDBValues)
                }
            })
            .collect::<Result<Vec<String>, BigError>>()?;

        Ok(FSFieldOutput {
            field_order: f.field_order,
            field_name: f.field_name.clone(),
            field_value: my_field_value.clone(),
            category_name: f.category_name.clone(),
            field_value_type: my_field_value_types.clone(),
        })
    }
}

impl From<FSField> for FSFieldOutput {
    fn from(value: FSField) -> Self {
        FSFieldOutput {
            field_order: value.field_order,
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
            field_order: value.field_order,
            field_name: value.field_name,
            field_value: value.field_value,
            category_name: value.category_name,
            field_value_type: value.field_value_type,
        }
    }
}

// #[derive(Serialize, Deserialize, Clone, InputObject)]
// pub struct FormStructure {
//     pub form_template_version: Vec<i32>,
//     pub form_id: Option<Uuid>,
//     pub enums: Option<Vec<EnumLists>>,
//     pub all_fields: Vec<FSField>,
// }

#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct FormStructure {
    pub form_template_version: Vec<i32>,
    pub form_id: Option<Uuid>,
    pub field_names: Vec<String>,
    pub category_names: Vec<String>,
    pub field_value_types: Vec<String>,
    pub enums: Vec<EnumLists>,
    pub all_fields: Vec<FSField>,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct FormStructureOutput {
    pub form_template_version: Vec<i32>,
    pub form_id: Option<Uuid>,
    pub field_names: Vec<String>,
    pub category_names: Vec<String>,
    pub field_value_types: Vec<String>,
    pub enums: Vec<EnumListsOutput>,
    pub all_fields: Vec<FSFieldOutput>,
}

impl From<FormStructure> for FormStructureOutput {
    fn from(value: FormStructure) -> Self {
        // let my_enums = match value.enums {
        //     Some(e) => Some(
        //         e.into_iter()
        //             .map(|e| EnumListsOutput::from(e))
        //             .collect::<Vec<EnumListsOutput>>(),
        //     ),
        //     None => None,
        // };
        let my_enums = value
            .enums
            .into_iter()
            .map(|e| EnumListsOutput::from(e))
            .collect::<Vec<EnumListsOutput>>();

        let my_fields = value
            .all_fields
            .into_iter()
            .map(|f| FSFieldOutput::from(f))
            .collect();

        FormStructureOutput {
            form_template_version: value.form_template_version,
            form_id: value.form_id,
            field_names: value.field_names,
            category_names: value.category_names,
            field_value_types: value.field_value_types,
            enums: my_enums,
            all_fields: my_fields,
        }
    }
}

impl From<FormStructureOutput> for FormStructure {
    fn from(value: FormStructureOutput) -> Self {
        // let my_enums = match value.enums {
        //     Some(e) => Some(
        //         e.into_iter()
        //             .map(|e| EnumLists::from(e))
        //             .collect::<Vec<EnumLists>>(),
        //     ),
        //     None => None,
        // };
        let my_enums = value
            .enums
            .into_iter()
            .map(|e| EnumLists::from(e))
            .collect::<Vec<EnumLists>>();

        let my_fields = value
            .all_fields
            .into_iter()
            .map(|f| FSField::from(f))
            .collect();

        FormStructure {
            form_template_version: value.form_template_version,
            form_id: value.form_id,
            field_names: value.field_names,
            category_names: value.category_names,
            field_value_types: value.field_value_types,
            enums: my_enums,
            all_fields: my_fields,
        }
    }
}

#[derive(InputObject, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EnumLists {
    enum_name: String,
    enums: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
pub struct EnumListsOutput {
    enum_name: String,
    enums: Vec<String>,
}

impl From<EnumLists> for EnumListsOutput {
    fn from(value: EnumLists) -> Self {
        EnumListsOutput {
            enum_name: value.enum_name,
            enums: value.enums,
        }
    }
}

impl From<EnumListsOutput> for EnumLists {
    fn from(value: EnumListsOutput) -> Self {
        EnumLists {
            enum_name: value.enum_name,
            enums: value.enums,
        }
    }
}

impl FormStructure {
    /*
        ALL form validation concerns happen here. Otherwise we are dealing with strings.
        This does make me think that JSON might be more applicable.
        Already this feels a lot better though.
    */
    pub fn validate_form(&self) -> Result<FormStructure, BigError> {
        use crate::dive_forms::current_form::{AllEnums, DisciplinesEnum, FieldValueTypes};
        let template = FormStructure::get_latest_form_template();

        let mut new_fields = vec![];

        // compare the category, field_names, enums and field_value_types
        if self.field_names != template.field_names
            || self.category_names != template.category_names
            || self.field_value_types != template.field_value_types
            || self
                .enums
                .clone()
                .into_iter()
                .map(|e| EnumListsOutput::from(e))
                .collect::<Vec<EnumListsOutput>>()
                != template.enums
        {
            return Err(BigError::FormFieldNotMatching);
        }

        for field in &self.all_fields {
            // Check that the category name and field name is valid against the templates options
            if !template.category_names.contains(&field.category_name)
                || !template.field_names.contains(&field.field_name)
            {
                return Err(BigError::FormFieldNotMatching);
            }

            // check that there is a related template field (also checking the field_name)
            let template_field = template
                .all_fields
                .iter()
                .find(|e| e.field_name == field.field_name);

            if let Some(template_field) = template_field {
                // check if the category and value types are equal
                if template_field.category_name != field.category_name
                    || template_field.field_value_type != field.field_value_type
                {
                    return Err(BigError::FormFieldNotMatching);
                }

                if let Some(all_values) = &field.field_value {
                    for (field_value, field_type) in
                        all_values.iter().zip(field.field_value_type.iter())
                    {
                        let val_ok = match FieldValueTypes::from_str(&field_type) {
                            Ok(field_type) => match field_type {
                                FieldValueTypes::Number => field_value
                                    .parse::<i32>()
                                    .map_err(|e| BigError::ParseIntError { source: e })
                                    .is_ok(),
                                // FieldValueTypes::Enum => template
                                //     .enums
                                //     .as_ref()
                                //     .and_then(|e| {
                                //         e.iter()
                                //             .find(|e| e.enum_name == field.enum_name)
                                //             .and_then(|e| Some(e.enums.contains(&field_value)))
                                //     })
                                //     .is_some(),
                                FieldValueTypes::Timestamp => NaiveDateTime::from_str(&field_value)
                                    .map_err(|e| BigError::ChronoParseError { source: e })
                                    .is_ok(),
                                FieldValueTypes::Interval => field_value
                                    .parse::<u64>()
                                    .map_err(|e| BigError::ParseIntError { source: e })
                                    .is_ok(),
                                FieldValueTypes::Text => true,
                            },
                            Err(_) => match AllEnums::from_str(&field_type) {
                                Ok(field_type) => match field_type {
                                    AllEnums::DisciplinesEnum => {
                                        DisciplinesEnum::from_str(&field_value).is_ok()
                                    }
                                },
                                Err(_) => false,
                            },
                        };

                        if !val_ok {
                            return Err(BigError::FormValueNotMatching {
                                val: field_value.clone(),
                            });
                        }
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
            field_names: template.field_names,
            category_names: template.category_names,
            field_value_types: template.field_value_types,
            enums: template
                .enums
                .into_iter()
                .map(|e| EnumLists::from(e))
                .collect::<Vec<EnumLists>>(),
            form_template_version: template.form_template_version,
            all_fields: new_fields,
        })
    }

    pub fn get_latest_form_template() -> FormStructureOutput {
        // Only use locally scoped variables
        // Client should ONLY receive strings. No enums.
        use crate::dive_forms::current_form::{
            AllEnums, CategoryNames, DisciplinesEnum, FieldNames, FieldValueTypes,
        };
        FormStructureOutput {
            form_template_version: vec![1, 0, 0],
            form_id: None,
            field_names: FieldNames::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            category_names: CategoryNames::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            field_value_types: FieldValueTypes::iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            enums: vec![EnumListsOutput {
                enum_name: AllEnums::DisciplinesEnum.to_string(),
                enums: DisciplinesEnum::iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
            }],
            all_fields: vec![
                (FSFieldOutput {
                    field_order: None,
                    field_value: None,
                    field_name: FieldNames::GeneralFeeling.to_string(),
                    category_name: CategoryNames::General.to_string(),
                    field_value_type: vec![FieldValueTypes::Number.to_string()],
                }),
                (FSFieldOutput {
                    field_order: None,
                    field_value: None,
                    field_name: FieldNames::MaxDepthWithDiscipline.to_string(),
                    category_name: CategoryNames::InWater.to_string(),
                    field_value_type: vec![
                        FieldValueTypes::Number.to_string(),
                        AllEnums::DisciplinesEnum.to_string(),
                    ],
                }),
                (FSFieldOutput {
                    field_order: None,
                    field_value: None,
                    field_name: FieldNames::CompletedFormName.to_string(),
                    category_name: CategoryNames::FormRelated.to_string(),
                    field_value_type: vec![FieldValueTypes::Text.to_string()],
                }),
            ],
        }
    }

    pub fn construct_from_forms(
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
                        field_names: structure.field_names,
                        category_names: structure.category_names,
                        field_value_types: structure.field_value_types,
                        enums: structure.enums,
                        all_fields,
                    },
                })
            })
            .collect::<Result<Vec<FormOutput>, BigError>>();
        form_structure_outputs
    }

    pub fn construct_from_completed_forms(
        forms: &Vec<CompletedForm>,
        fields: &Vec<CompletedFormField>,
    ) -> Result<Vec<FormStructureOutput>, BigError> {
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

                Ok(FormStructureOutput {
                    form_template_version: version,
                    form_id: Some(form.id),
                    field_names: structure.field_names,
                    category_names: structure.category_names,
                    field_value_types: structure.field_value_types,
                    enums: structure.enums,
                    all_fields,
                })
            })
            .collect::<Result<Vec<FormStructureOutput>, BigError>>();
        form_structure_outputs
    }
}
