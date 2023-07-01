use async_graphql::{OneofObject, Union};

use super::formV1::form::{FormInputV1, FormOutputV1};

// NOTE: This is only for receiving from the client
#[derive(OneofObject)]
pub enum AllFormsInput {
    V1(FormInputV1),
}

// All operations are done on this object
#[derive(Union)]
pub enum AllFormsOutput {
    V1(FormOutputV1),
}
