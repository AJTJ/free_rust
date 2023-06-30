use async_graphql::{InputObject, InputType, OutputType, SimpleObject};

// VERSIONED ENDPOINT TBD

use super::{
    form_1::{FormInputV1, FormOutputV1},
    form_trait::{FormInputNew, FormTrait},
};

pub type FormVersion = Vec<i32>;

#[derive(SimpleObject)]
pub struct AllFormVersions {
    v_1: FormOutputV1,
}

#[derive(InputObject)]
pub struct AllFormVersionsInput {
    v_1: Option<FormInputNew<FormInputV1>>,
}

// impl AllFormVersionsInput {
//     fn get_array<T: FormTrait + InputType>(&self) -> [Option<T>; 1] {
//         [self.v_1.and_then(|c| Some(c.inner))]
//     }
// }
