use async_graphql::InputObject;

#[derive(InputObject, Clone)]
pub struct QueryParams {
    pub after: Option<String>,
    // DELAYING THIS pub before: Option<String>,
    pub first: Option<usize>,
    // DELAYING THIS pub last: Option<i 32>,
}
