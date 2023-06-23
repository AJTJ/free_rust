use async_graphql::*;
use async_graphql::{types::connection::*, Context, EmptySubscription, Object, Schema};
use diesel::query_dsl::methods::FilterDsl;
use diesel::QueryDsl;
use tracing_subscriber::registry::SpanData;
use uuid::Uuid;

pub trait HasID {
    fn id(&self) -> String;
}

pub async fn gql_query<O, T>(
    ctx: &Context<'_>,
    // This is an opaque string of the "createdAt" data
    after: Option<String>,
    // DELAYING THIS before: Option<String>,
    // EX: the first 10 of something
    first: Option<i32>,
    // DELAYING THIS last: Option<i 32>,
    // this function will implement postgres queries
    db_retrieval_fn: &dyn Fn(&Context<'_>, T, Uuid, Option<String>, Option<usize>) -> Vec<O>,
    table: T,
    user_id: Uuid,
) -> Result<Connection<String, O, EmptyFields, EmptyFields>>
where
    O: OutputType + HasID,
    T: QueryDsl,
{
    query(
        after,
        None,
        first,
        None,
        |after: Option<String>, before, first, last| async move {
            // this should return the `first` amount of items after the `after` createdAt date
            let vec_of_items = db_retrieval_fn(ctx, table, user_id, after, first);

            let mut connection = Connection::new(
                // for pagination purposes, I do need the "start" and "end" for here
                // how does one do that without retrieving everything from the db in the first place?
                // can the db_retrieval_fn get the total length of my query before pagination?
                true, /* should tell if there was a previous page */
                true, /* should tell if there's a next page */
            );
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
