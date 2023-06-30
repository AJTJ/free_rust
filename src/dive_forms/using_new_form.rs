use crate::dive_forms::new_form_idea::FormInput1_0_0;
use crate::dive_forms::new_form_idea_2::FormInput2_0_0;
use crate::{dive_forms::new_form_idea, errors::BigError};
use async_graphql::{InputObject, InputType, OutputType, SimpleObject};
use serde::{Deserialize, Serialize};
pub trait FormTrait {
    fn return_form(&self) -> Self;
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
pub struct FormOutput<T: FormTrait + OutputType> {
    inner: T,
}

impl<T: FormTrait + OutputType> FormOutput<T> {
    // would this be necessary?
    pub fn validate_form(&self) -> Result<T, BigError> {
        // let versioned = self.get_versioned_template(&self);
        // some way to sort the different forms via files
        // compare_forms(self.inner, versioned)
        unimplemented!()
    }

    pub fn get_versioned_template(&self) -> T {
        // Some way to return a template of a form
        // how would this happen?
        unimplemented!()
    }

    pub fn construct_from_database(db_info: i32) -> Self {
        // do things here
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, InputObject, Clone, Copy)]
#[graphql(concrete(name = "FormInput1_0_0", params(FormInput1_0_0)))]
#[graphql(concrete(name = "FormInput2_0_0", params(FormInput2_0_0)))]
pub struct FormInput<T: FormTrait + InputType> {
    inner: T,
}

impl<T: FormTrait + InputType> FormInput<T> {
    // would this be necessary?
    pub fn validate_form(&self) -> Result<T, BigError> {
        // let versioned = self.get_versioned_template(&self);
        // some way to sort the different forms via files
        // compare_forms(self.inner, versioned)
        unimplemented!()
    }

    pub fn get_versioned_template(&self) -> T {
        // Some way to return a template of a form
        // how would this happen?
        unimplemented!()
    }

    pub fn construct_from_database(db_info: i32) -> Self {
        // do things here
        unimplemented!()
    }
}
