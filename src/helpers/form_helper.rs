use std::mem::discriminant;

use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AllCustomEnums {}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum InputTypes {
    Number,
    CustomEnums,
    Timestamp,
    Interval,
    Text,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum AllInputNames {
    GeneralFeeling(Option<FormInput>),
    // CustomInput((String, Option<FormInput>)),
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum AllCategoryNames {
    General,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct FormInput {
    input_order: Option<u32>,
    // input_name: AllInputNames,
    category_name: AllCategoryNames,
    input_type: InputTypes,
}

/*
WHAT IS HAPPENING
 - Send the user a filled out json object derived from the form_structure
 - Receive a list of input names
 - Create a new form from the input names, but derived from the form_structure
 - Why not just store this new form in the database as is?
 - The front end should already have the mechanisms for parsing the FormTemplate for generating a
*/

#[derive(Serialize, Deserialize)]
pub struct FormTemplate {
    // pub general_feeling: Option<FormInput>,
    pub all_inputs: Vec<AllInputNames>,
}

impl FormTemplate {
    pub fn validate_form(inc_inputs: Vec<AllInputNames>) -> FormTemplate {
        let predefined_structure = crate::helpers::form_helper::FormTemplate::get_form_structure();
        let output: FormTemplate = {
            let mut all_inputs = vec![];

            inc_inputs.iter().for_each(|i| {
                println!("HIT");
                let el = predefined_structure
                    .all_inputs
                    .iter()
                    .find(|e| discriminant(*e) == discriminant(i));

                if let Some(el) = el {
                    all_inputs.push(*el);
                }
            });

            FormTemplate { all_inputs }
        };
        output
    }

    pub fn get_form_structure() -> FormTemplate {
        FormTemplate {
            all_inputs: vec![
                (AllInputNames::GeneralFeeling(Some(FormInput {
                    input_order: None,
                    // input_name: AllInputNames::GeneralFeeling,
                    category_name: AllCategoryNames::General,
                    input_type: InputTypes::Number,
                }))),
            ],
        }
    }

    pub fn get_form_structure_json() -> Value {
        json!(&Self::get_form_structure())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn form_lifecycle() {
        use crate::helpers::form_helper::*;
        // user gets sent this json object of all possible form input fields and their values etc...
        let json_form = FormTemplate::get_form_structure_json();
        println!("{}", serde_json::to_string_pretty(&json_form).unwrap());
        // user returns a list of AllInputNames enums
        let return_val = vec![AllInputNames::GeneralFeeling(None)];
        // get new ordered (from vec)
        let new_form = FormTemplate::validate_form(return_val);

        // and now with this new form, I can either store it in the database in json (simplest for now)
        // or I can break it out into inputs
    }
}
