use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use bb8;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use free_rust::graphql_schema::{DiveQLSchema, MutationRoot, QueryRoot, Storage};
use std::env;

// tracing
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

async fn index(schema: web::Data<DiveQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // graphql schema builder
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8080");
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("no DB URL");

    //BB8 Pool
    // let manager = bb8_po
    // let pool = bb8::Pool::builder().build(manager).await.unwrap();

    // R2D2 pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Tracing
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("start of service");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(schema.clone()))
            .wrap(Logger::default())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
