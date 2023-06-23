use crate::errors::BigError;
use async_graphql::types::connection::*;
use async_graphql::*;
use futures_util::Future;
use uuid::Uuid;

// This is an opaque string of the "createdAt" data
// DELAYING THIS before: Option<String>,
// EX: the first 10 of something
// DELAYING THIS last: Option<i 32>,
// this function will implement postgres queries

pub async fn gql_query<
    O: OutputType,
    R: Future<Output = Result<Vec<(String, O)>, BigError>>,
    F: Fn(Option<String>, Option<usize>) -> R,
>(
    after: Option<String>,
    first: Option<i32>,
    db_retrieval_closure: F,
) -> Result<Connection<String, O, EmptyFields, EmptyFields>> {
    query(
        after,
        None,
        first,
        None,
        |after: Option<String>, before, first, last| async move {
            // this should return the `first` amount of items after the `after` createdAt date
            let vec_of_items = db_retrieval_closure(after, first).await?;
            // TODO: the queries need to return tuples with their ids or I need to implement a new `gql_query` for every type that uses it.
            // not sure which one makes the most sense rn.
            // TODO: the queries need to return a minimum of pagination information

            let mut connection = Connection::new(true, true);
            connection.edges.extend(
                vec_of_items
                    .into_iter()
                    .map(|(u, o)| Edge::with_additional_fields(u, o, EmptyFields)),
            );
            Ok::<_, async_graphql::Error>(connection)
        },
    )
    .await
}

// pub fn query_wrapper<T, O>(
//     ctx: &Context<'_>,
//     table: T,
//     user_id: Uuid,
//     after: Option<String>,
//     first: Option<usize>,
// ) -> usize
// where
//     O: OutputType + HasID,
//     T: FilterDsl<O>,
// {
//     let first = match first {
//         Some(v) => std::cmp::max(v, 100),
//         None => 100,
//     };

//     // This could be done outside
//     let q = table.filter(table.user_id.eq(user_id));

//     if let Some(a) = after {};

//     q = q.limit(first);

//     42
// }
