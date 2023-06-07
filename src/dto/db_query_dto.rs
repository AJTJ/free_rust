use async_graphql::InputObject;

#[derive(InputObject)]
pub struct DBQueryObject {
    pub limit: Option<i32>,
}
