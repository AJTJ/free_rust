// use crate::graphql_schema::DbPool;
// use crate::utility::errors::ActixBlockingSnafu;
// use crate::utility::errors::BigError;
// use crate::utility::gql::query_dto::QueryParams;
// use actix_web::web;
// use async_graphql::async_trait;
// use async_graphql::dataloader::*;
// use snafu::ResultExt;
// use std::collections::HashMap;
// use std::sync::Arc;
// use uuid::Uuid;

// use super::actions::get_dive_sessions;
// use super::dto::dive_session_dto::ApnesSessionRetrievalData;
// use super::dto::dive_session_dto::DiveSession;

// pub struct ApneaSessionLoader(DbPool);

// impl ApneaSessionLoader {
//     pub fn new(postgres_pool: DbPool) -> Self {
//         Self(postgres_pool)
//     }
// }

// #[async_trait::async_trait]
// impl Loader<Uuid> for ApneaSessionLoader {
//     type Value = DiveSession;
//     type Error = Arc<BigError>;

//     async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
//         let pool = self.0.clone();
//         let my_keys = keys.to_vec();
//         let output = web::block(move || {
//             let mut conn = pool.get().unwrap();
//             get_dive_sessions(
//                 &mut conn,
//                 ApnesSessionRetrievalData::Sessions(my_keys),
//                 None,
//                 // TODO: I don't know if this makes sense
//                 QueryParams {
//                     after: None,
//                     first: None,
//                 },
//             )
//         })
//         .await
//         .context(ActixBlockingSnafu)??;

//         // it seems like it is required to return a hashmap?
//         let mut m: HashMap<Uuid, DiveSession> = HashMap::new();
//         for d in output.into_iter() {
//             m.insert(d.id, d);
//         }
//         Ok(m)
//     }
// }
