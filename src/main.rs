#![feature(async_closure)]
#[macro_use]
// mods
extern crate diesel;
pub mod actions;
pub mod auth_data;
pub mod data_loaders;
pub mod dive_forms;
pub mod dto;
pub mod env_data;
pub mod errors;
pub mod graphql_query;
pub mod graphql_schema;
pub mod guards;
pub mod helpers;
pub mod schema;
pub mod token_source;

use actix_web::{
    guard,
    http::header::{HeaderMap, AUTHORIZATION, COOKIE},
    middleware::{self, Logger},
    rt,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};

use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use data_loaders::DiveSessionsLoader;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use env_data::SharedVars;
use graphql_schema::{DbPool, DiveQLSchema, Mutation, Query};
use r2d2;
use redis::Client;
use std::env;
use token_source::Token;

// tracing
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    let my_header = headers
        .get(AUTHORIZATION)
        .or_else(|| headers.get(COOKIE))
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok());
    my_header
}

async fn index(
    schema: web::Data<DiveQLSchema>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();

    // THIS GRABS THE AUTHORIZATION TOKEN CORRECTLY
    // let auth_header_value = http_req.headers().get(http::header::AUTHORIZATION);
    // info!("AUTH HEADER: {:?}", auth_header_value);
    // let cookie_header_value = http_req.headers().get(http::header::COOKIE);
    // info!("COOKIE HEADER: {:?}", cookie_header_value);
    // info!("auth_header_value: {auth_header_value:?}");

    if let Some(token) = get_token_from_headers(http_req.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("no DB URL");
    let redis_url = env::var("REDIS_URL").expect("no redis URL");
    let environment = env::var("ENVIRONMENT").expect("no environment variable");

    // Pooled database
    let db_connection_manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(db_url);
    let pooled_database: DbPool = diesel::r2d2::Pool::builder()
        // .max_size(1)
        .build(db_connection_manager)
        .expect("Failed to create pool.");

    // Tracing
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // REDIS
    let redis_client = Client::open(redis_url).expect("failure starting redis server");
    let redis_pool = r2d2::Pool::new(redis_client).unwrap();

    let env_vars = SharedVars { environment };

    // graphql schema builder
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(redis_pool)
        .data(DataLoader::new(
            DiveSessionsLoader::new(pooled_database.clone()),
            rt::spawn,
        ))
        .data(pooled_database.clone())
        .data(env_vars)
        .limit_depth(8)
        .finish();

    // println!("{}", &schema.sdl());

    info!("start of service - Playground: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(Data::new(schema.clone()))
            .wrap(Logger::default())
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    // Limit workers for testing
    // .workers(1)
    // This would ONLY be available on the local machine
    // .bind("127.0.0.1:8080")?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

/*
   OTHER

   BB8 Pool
   let manager = bb8_po
   let pool = bb8::Pool::builder().build(manager).await.unwrap();

   .wrap(
                IdentityMiddleware::builder()
                    .login_deadline(Some(Duration::from_secs(604800)))
                    .build(),
            )
*/

// pub async fn index(
//     schema: web::Data<DiveQLSchema>,
//     req: GraphQLRequest,
//     session: Session,
// ) -> GraphQLResponse {
//     // TODO: get the session_id from the request

//     // get the session data from the request
//     // let uid = session.get::<String>("user_id").unwrap_or(None);

//     // build the session data
//     // let id = Identity { id: uid };

//     // send the session data through to the gql schema
//     let session = Shared::new(session);
//     schema.execute(req.into_inner().data(session)).await.into()
// }
