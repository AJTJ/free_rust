use async_graphql::InputObject;

#[derive(InputObject)]
pub struct DBQueryParams {
    pub limit: Option<i32>,
}
