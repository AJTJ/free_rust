use async_graphql::InputObject;

#[derive(InputObject, Clone)]
pub struct QueryParams {
    pub limit: Option<i32>,
}
