use async_graphql::*;
use async_graphql::{types::connection::*, Context, EmptySubscription, Object, Schema};
use tracing_subscriber::registry::SpanData;
use uuid::Uuid;

pub trait HasID {
    fn id(&self) -> String;
}

pub async fn gql_query<T>(
    // This is an opaque string of the "createdAt" data
    after: Option<String>,
    // DELAYING THIS before: Option<String>,
    // EX: the first 10 of something
    first: Option<i32>,
    // DELAYING THIS last: Option<i 32>,
    // this function will implement postgres queries
    db_retrieval_fn: &dyn Fn(Option<String>, Option<usize>) -> Vec<T>,
) -> Result<Connection<String, T, EmptyFields, EmptyFields>>
where
    T: OutputType + HasID,
{
    query(
        after,
        None,
        first,
        None,
        |after: Option<String>, before, first, last| async move {
            // this should return the `first` amount of items after the `after` createdAt date
            let vec_of_items = db_retrieval_fn(after, first);

            // for pagination purposes, I do need the "start" and "end" to generate the pages for gql
            // how does one do that without retrieving everything in the first place?
            // can the db_retrieval_fn get the total length of my query before pagination?

            let mut connection = Connection::new(true, true);
            connection.edges.extend(
                vec_of_items
                    .into_iter()
                    .map(|n| Edge::with_additional_fields(n.id(), n, EmptyFields)),
            );
            Ok::<_, async_graphql::Error>(connection)
        },
    )
    .await
}
