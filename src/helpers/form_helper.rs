use std::str::FromStr;

use async_graphql::{Enum, InputObject, ID};
use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use strum::{Display, EnumString};

use crate::{actions::get_logger_by_id, errors::BigError};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AllCustomEnums {}

#[derive(Enum, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, EnumString, Display, Eq)]
pub enum InputTypes {
    Number,
    CustomEnums,
    Timestamp,
    Interval,
    Text,
}

#[derive(Enum, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Debug, EnumString, Display)]
pub enum FormInputNames {
    GeneralFeeling,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, EnumString, Display, Enum, Eq)]
pub enum AllCategoryNames {
    General,
    // there will be more
}

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
pub struct Field {
    pub input_name: FormInputNames,
    pub input_value: Option<String>,
    pub category_name: AllCategoryNames,
    pub input_type: InputTypes,
}

#[derive(InputObject, Serialize, Deserialize)]
pub struct Form {
    pub form_template_version: f64,
    pub form_used: Option<ID>,
    pub enums: Option<Vec<EnumLists>>,
    pub all_fields: Vec<Field>,
}

#[derive(InputObject, Serialize, Deserialize)]
pub struct EnumLists {
    field_name: FormInputNames,
    enums: Vec<String>,
}

impl Form {
    /*
    the user is creating or editin a form, and we replace it with our own preset values
       TODO: Versioning
       - if the user is updating an old form that uses an older form template, we need to take that into account.
    */
    pub fn validate_form(&self) -> Result<Form, BigError> {
        let template = Form::get_versioned_form_template(self.form_template_version);

        let mut new_fields = vec![];

        for field in self.all_fields {
            // check that there is a related template field
            let template_field = template
                .all_fields
                .iter()
                .find(|e| e.input_name == field.input_name);

            if let Some(template_field) = template_field {
                if template_field.category_name != field.category_name
                    || template_field.input_type != field.input_type
                {
                    return Err(BigError::FormFieldNotMatching);
                }

                if let Some(val) = field.input_value {
                    let val_ok = match field.input_type {
                        InputTypes::Number => val
                            .parse::<i32>()
                            .map_err(|e| BigError::ParseIntError { source: e })
                            .is_ok(),
                        InputTypes::CustomEnums => template
                            .enums
                            .and_then(|e| {
                                e.iter()
                                    .find(|e| e.field_name == field.input_name)
                                    .and_then(|e| Some(e.enums.contains(&val)))
                            })
                            .is_some(),

                        InputTypes::Timestamp => NaiveDateTime::from_str(&val)
                            .map_err(|e| BigError::ParseError { source: e })
                            .is_ok(),
                        InputTypes::Interval => val
                            .parse::<u64>()
                            .map_err(|e| BigError::ParseIntError { source: e })
                            .is_ok(),
                        InputTypes::Text => true,
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
        Ok(Form {
            form_used: self.form_used,
            enums: template.enums,
            form_template_version: template.form_template_version,
            all_fields: new_fields,
        })
    }

    // TODO: Probably get this from JSON/DOCUMENTATION files
    pub fn get_versioned_form_template(version: f64) -> Form {
        Form {
            form_template_version: 1.0,
            form_used: None,
            enums: None,
            all_fields: vec![
                (Field {
                    input_value: None,
                    input_name: FormInputNames::GeneralFeeling,
                    category_name: AllCategoryNames::General,
                    input_type: InputTypes::Number,
                }),
            ],
        }
    }
    // TODO: Probably get this from JSON/DOCUMENTATION files
    pub fn get_latest_form_template() -> Form {
        Form {
            form_template_version: 1.0,
            form_used: None,
            enums: None,
            all_fields: vec![
                (Field {
                    input_value: None,
                    input_name: FormInputNames::GeneralFeeling,
                    category_name: AllCategoryNames::General,
                    input_type: InputTypes::Number,
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
        let json_form = Form::get_latest_form_template();

        // client returns a custom form
        let custom_form = Form {
            form_template_version: 1.0,
            form_used: None,
            enums: None,
            all_fields: vec![Field {
                input_value: None,
                input_name: FormInputNames::GeneralFeeling,
                category_name: AllCategoryNames::General,
                input_type: InputTypes::Number,
            }],
        };

        // get new form based on the enums from the client
        let validated_new_form = custom_form.validate_form();

        // they create a completed form with it
        let completed_form = Form {
            form_template_version: 1.0,
            form_used: None,
            enums: None,
            all_fields: vec![Field {
                input_value: Some("100".to_string()),
                input_name: FormInputNames::GeneralFeeling,
                category_name: AllCategoryNames::General,
                input_type: InputTypes::Number,
            }],
        };

        // this new form is validated

        let validated_completed_form = completed_form.validate_form().unwrap();

        // and is then stored in the database
    }
}
