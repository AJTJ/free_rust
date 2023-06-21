use std::str::FromStr;

use async_graphql::{Enum, InputObject, ID};
use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use strum::{Display, EnumString};

use crate::{actions::get_form_by_id, errors::BigError};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AllCustomEnums {}

#[derive(Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq)]
pub enum FieldValueTypes {
    Number,
    CustomEnums,
    Timestamp,
    Interval,
    Text,
}

#[derive(Enum, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Debug, EnumString, Display)]
pub enum FieldNames {
    GeneralFeeling,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq)]
pub enum CategoryNames {
    General,
    // there will be more
}

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
pub struct FSField {
    pub field_name: FieldNames,
    pub field_value: Option<String>,
    pub category_name: CategoryNames,
    pub field_value_type: FieldValueTypes,
}

#[derive(InputObject, Serialize, Deserialize)]
pub struct FormStructure {
    pub form_template_version: f64,
    pub form_used: Option<ID>,
    pub enums: Option<Vec<EnumLists>>,
    pub all_fields: Vec<FSField>,
}

#[derive(InputObject, Serialize, Deserialize)]
pub struct EnumLists {
    field_name: FieldNames,
    enums: Vec<String>,
}

impl FormStructure {
    /*
    the user is creating or editin a form, and we replace it with our own preset values
       TODO: Versioning
       - if the user is updating an old form that uses an older form template, we need to take that into account.
    */
    pub fn validate_form(&self) -> Result<FormStructure, BigError> {
        let template = FormStructure::get_versioned_form_template(self.form_template_version);

        let mut new_fields = vec![];

        for field in self.all_fields {
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

                if let Some(val) = field.field_value {
                    let val_ok = match field.field_value_type {
                        FieldValueTypes::Number => val
                            .parse::<i32>()
                            .map_err(|e| BigError::ParseIntError { source: e })
                            .is_ok(),
                        FieldValueTypes::CustomEnums => template
                            .enums
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
                    };
                    if !val_ok {
                        return Err(BigError::FormValueNotMatching);
                    }
                }

                // push the input form to the new fields, thus keeping the value if it is completed
                new_fields.push(field)
            } else {
                // no related template field
                return Err(BigError::FormFieldNotMatching);
            }
        }

        // Does this make sense?
        Ok(FormStructure {
            form_used: self.form_used,
            enums: template.enums,
            form_template_version: template.form_template_version,
            all_fields: new_fields,
        })
    }

    // TODO: Probably get this from JSON/DOCUMENTATION files
    pub fn get_versioned_form_template(version: f64) -> FormStructure {
        FormStructure {
            form_template_version: 1.0,
            form_used: None,
            enums: None,
            all_fields: vec![
                (FSField {
                    field_value: None,
                    field_name: FieldNames::GeneralFeeling,
                    category_name: CategoryNames::General,
                    field_value_type: FieldValueTypes::Number,
                }),
            ],
        }
    }
    // TODO: Probably get this from JSON/DOCUMENTATION files
    pub fn get_latest_form_template() -> FormStructure {
        FormStructure {
            form_template_version: 1.0,
            form_used: None,
            enums: None,
            all_fields: vec![
                (FSField {
                    field_value: None,
                    field_name: FieldNames::GeneralFeeling,
                    category_name: CategoryNames::General,
                    field_value_type: FieldValueTypes::Number,
                }),
            ],
        }
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
        use crate::helpers::form_helper::*;

        // user gets the latest form template fields and their values etc...
        // crucially this is everything they need to save and use forms
        let json_form = FormStructure::get_latest_form_template();

        // client returns a custom form
        let custom_form = FormStructure {
            form_template_version: 1.0,
            form_used: None,
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
            form_template_version: 1.0,
            form_used: None,
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
