use crate::dto::query_dto::QueryParams;
use crate::errors::BigError;
use async_graphql::types::connection::*;
use async_graphql::*;
use futures_util::Future;

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

// use crate::dto::query_dto::QueryParams;
// use crate::errors::BigError;
// use async_graphql::types::connection::*;
// use async_graphql::*;
// use futures_util::Future;

// pub async fn gql_query<
//     O: OutputType,
//     R: Future<Output = Result<Vec<(String, O)>, BigError>>,
//     F: Fn(QueryParams) -> R,
// >(
//     query_params: QueryParams,
//     db_retrieval_closure: F,
// ) -> Result<Connection<String, O, EmptyFields, EmptyFields>> {
//     query(
//         query_params.after,
//         None,
//         query_params.first.and_then(|n| Some(n as i32)),
//         None,
//         |after: Option<String>, _before, first, _last| async move {
//             // NOTE: The String is the Opaque string, representing the Date rn.
//             let vec_of_items = db_retrieval_closure(QueryParams { after, first }).await?;
//             let mut connection = Connection::new(true, true);
//             connection.edges.extend(
//                 vec_of_items
//                     .into_iter()
//                     .map(|(u, o)| Edge::with_additional_fields(u, o, EmptyFields)),
//             );
//             Ok::<_, async_graphql::Error>(connection)
//         },
//     )
//     .await
// }
