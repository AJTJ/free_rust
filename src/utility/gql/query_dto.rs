use async_graphql::InputObject;
use uuid::Uuid;

#[derive(InputObject, Clone)]
pub struct QueryParams {
    // NOTE: This is an OpaqueString representing a DateTime (usually)
    pub after: Option<String>,
    // DELAYING THIS pub before: Option<String>,
    pub first: Option<usize>,
    // DELAYING THIS pub last: Option<i 32>,
}
