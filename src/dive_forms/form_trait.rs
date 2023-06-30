use crate::dive_forms::form_1::FormInputV1;
use crate::errors::BigError;
use async_graphql::{InputObject, InputType, OutputType, SimpleObject};
use serde::{Deserialize, Serialize};

use super::helpers::FormVersion;
pub trait FormTrait {
    fn return_form(&self) -> Self;
    fn validate_form(&self) -> Self;
    fn get_template() -> Self;
}

#[derive(Serialize, Deserialize, SimpleObject, Clone, Copy)]
pub struct FormOutputNew<T: FormTrait + OutputType> {
    inner: T,
}

impl<T: FormTrait + OutputType> FormOutputNew<T> {
    // would this be necessary?
    pub fn validate_form(&self) -> Result<T, BigError> {
        // let versioned = self.get_versioned_template(&self);
        // some way to sort the different forms via files
        // compare_forms(self.inner, versioned)
        unimplemented!()
    }

    pub fn get_template(version: FormVersion) -> T {
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
#[graphql(concrete(name = "FormInputV_1", params(FormInputV1)))]
pub struct FormInputNew<T: FormTrait + InputType> {
    pub inner: T,
}

impl<T: FormTrait + InputType> FormInputNew<T> {
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
