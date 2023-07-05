use async_graphql::types::connection::*;
use async_graphql::*;
use futures_util::Future;

use crate::utility::errors::BigError;

use super::query_dto::QueryParams;

pub async fn gql_query<
    O: OutputType,
    R: Future<Output = Result<Connection<String, O>, BigError>>,
    F: Fn(QueryParams) -> R,
>(
    query_params: QueryParams,
    connection_retrieval_fn: F,
) -> Result<Connection<String, O, EmptyFields, EmptyFields>> {
    query(
        query_params.after,
        None,
        query_params.first.and_then(|n| Some(n as i32)),
        None,
        |after: Option<String>, _before, first, _last| async move {
            let connection = connection_retrieval_fn(QueryParams { after, first }).await?;
            Ok::<_, async_graphql::Error>(connection)
        },
    )
    .await
}
