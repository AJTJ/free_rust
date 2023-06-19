use async_graphql::{Enum, InputObject};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::mem::discriminant;

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

// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
// pub enum AllInputNames {
//     GeneralFeeling(Option<FormInput>),
//     // there will be more...
// }

#[derive(Enum, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Debug)]
pub enum FormInputNames {
    GeneralFeeling,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum AllCategoryNames {
    General,
    // there will be more
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct FormInput {
    // input_order: Option<u32>,
    // input_name: AllInputNames,
    category_name: AllCategoryNames,
    input_type: InputTypes,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct AltStruct {
    pub input_name: FormInputNames,
    pub form_input: FormInput,
}

#[derive(Serialize, Deserialize)]
pub struct FormTemplate {
    // pub general_feeling: Option<FormInput>,
    pub all_inputs: Vec<AltStruct>,
}

pub type UserFormInput = Vec<FormInputNames>;

impl FormTemplate {
    pub fn validate_form(inc_inputs: UserFormInput) -> FormTemplate {
        let predefined_structure = crate::helpers::form_helper::FormTemplate::get_form_structure();
        let output: FormTemplate = {
            let mut all_inputs = vec![];

            inc_inputs.iter().for_each(|i| {
                if let Some(matching_input) = predefined_structure
                    .all_inputs
                    .iter()
                    .find(|e| discriminant(&e.input_name) == discriminant(i))
                {
                    all_inputs.push(*matching_input);
                }
            });

            FormTemplate { all_inputs }
        };
        output
    }

    pub fn get_form_structure() -> FormTemplate {
        FormTemplate {
            all_inputs: vec![
                (AltStruct {
                    input_name: FormInputNames::GeneralFeeling,
                    form_input: FormInput {
                        // input_order: None,
                        // input_name: AllInputNames::GeneralFeeling,
                        category_name: AllCategoryNames::General,
                        input_type: InputTypes::Number,
                    },
                }),
            ],
        }
    }

    pub fn get_form_structure_json() -> Value {
        json!(&Self::get_form_structure())
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

        // user gets sent this json object of all possible form input fields and their values etc...
        let json_form = FormTemplate::get_form_structure_json();

        // client returns a list of AllInputNames enums
        let return_val = vec![FormInputNames::GeneralFeeling];

        // get new form based on the enums from the client
        let new_form = FormTemplate::validate_form(return_val);

        // and now with this new form, I can either store it in the database as a json blob (simplest for now)
        // or I can break it out into logger entries
    }
}
