use async_graphql::InputObject;

#[derive(InputObject)]
pub struct QueryParams {
    pub limit: Option<i32>,
}
