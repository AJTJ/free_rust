// use std::time::Duration;

// use crate::auth_data::SessionData;
// use crate::Shared;
// use actix_session::Session;
// use actix_web::web;
// use async_graphql::Context;
// use tracing::info;

// pub fn other_add_to_sesh(ctx: &Context<'_>, session_data: SessionData, encoded_session_id: String) {
//     info!("pre sesh2");
//     let shared_session = ctx.data::<Shared<Session>>().unwrap().clone();
//     std::thread::sleep(Duration::from_secs(5));
//     shared_session
//         .insert(encoded_session_id, session_data)
//         .unwrap();

//     info!("post sesh2");
//     // web::block(move || {
//     //     shared_session
//     //         .insert(encoded_session_id, session_data)
//     //         .unwrap();
//     // })
//     // .await
//     // .expect("failure to store in session");
// }
