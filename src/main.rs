use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{cookie::Key, App, HttpResponse, HttpServer};
use actix_web::{guard, web, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;

use send_wrapper::SendWrapper;
use std::env;
use std::ops::Deref;

// mods
#[macro_use]
extern crate diesel;
pub mod actions;
pub mod data;
pub mod graphql_schema;
pub mod schema;
pub mod session_data;

use graphql_schema::{DbPool, DiveQLSchema, Identity, MutationRoot, QueryRoot};

// TODO: Is this baaad?
#[derive(Clone, Debug)]
pub struct Shared<T>(pub Option<SendWrapper<T>>);

impl<T> Shared<T> {
    pub fn new(v: T) -> Self {
        Self(Some(SendWrapper::new(v)))
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0.as_deref().clone().unwrap()
    }
}

// tracing
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

pub async fn index(
    schema: web::Data<DiveQLSchema>,
    req: GraphQLRequest,
    session: Session,
) -> GraphQLResponse {
    // TODO: get the session_id from the request

    // get the session data from the request
    // let uid = session.get::<String>("user_id").unwrap_or(None);

    // build the session data
    // let id = Identity { id: uid };

    // send the session data through to the gql schema
    let session = Shared::new(session);
    schema.execute(req.into_inner().data(session)).await.into()
}

// async fn index(schema: web::Data<DiveQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("no DB URL");
    let redis_url = env::var("REDIS_URL").expect("no redis URL");

    // R2D2 pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool: DbPool = Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.");

    // Tracing
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Session
    let secret_key = Key::generate();

    // graphql schema builder
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

    info!("start of service - Playground: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            // .wrap(
            //     IdentityMiddleware::builder()
            //         .login_deadline(Some(Duration::from_secs(604800)))
            //         .build(),
            // )
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new(redis_url.clone()),
                secret_key.clone(),
            ))
            .wrap(Logger::default())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*
   OTHER
   BB8 Pool
   let manager = bb8_po
   let pool = bb8::Pool::builder().build(manager).await.unwrap();
*/
