#![feature(async_closure)]
#[macro_use]
// mods
extern crate diesel;
pub mod apnea_forms;
pub mod apnea_sessions;
pub mod auth;
pub mod env_data;
pub mod graphql_schema;
pub mod schema;
pub mod utility;
use crate::apnea_forms::form_loader::FormLoader;
use crate::apnea_sessions::{
    apnea_session_loader::ApneaSessionLoader, dive_loader_by_session::DiveLoaderBySession,
    dive_loader_by_user::DiveLoaderByUser,
};
use actix_web::{
    guard,
    http::header::{HeaderMap, AUTHORIZATION, COOKIE},
    middleware::{self, Logger},
    rt,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use async_graphql::async_trait;
use async_graphql::{
    dataloader::DataLoader,
    extensions::{
        ApolloTracing, Extension, ExtensionContext, ExtensionFactory, NextRequest, Tracing,
    },
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Response, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use auth::utility::token_source::Token;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use env_data::SharedEnvVars;
use graphql_schema::{DbPool, DiveQLSchema, Mutation, Query};
use r2d2;
use redis::Client;
use std::{env, sync::Arc};

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

// TODO: This seems like the place to put the user_id into the gql context.
async fn index(
    schema: web::Data<DiveQLSchema>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();

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
    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    // will be written to stdout.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // REDIS
    let redis_client = Client::open(redis_url).expect("failure starting redis server");
    let redis_pool = r2d2::Pool::new(redis_client).unwrap();

    let env_vars = SharedEnvVars { environment };

    struct AuthExtension;

    #[async_trait::async_trait]
    impl Extension for AuthExtension {
        async fn request(&self, ctx: &ExtensionContext<'_>, next: NextRequest<'_>) -> Response {
            // let token = ctx.data::<Token>();
            // info!("Auth Middleware experiemnt token: {token:?}");

            // let el = ctx.session_data.insert("meme");

            // The code here will be run before the prepare_request is executed.
            let result = next.run(ctx).await;

            // The code after the completion of this future will be after the processing, just before sending the result to the user.
            result
        }
    }

    struct AuthMiddleware;

    impl ExtensionFactory for AuthMiddleware {
        fn create(&self) -> std::sync::Arc<dyn async_graphql::extensions::Extension> {
            let auth_extension = AuthExtension {};

            Arc::new(auth_extension)
        }
    }

    // graphql schema builder
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(Tracing)
        .extension(ApolloTracing)
        .data(redis_pool)
        .extension(AuthMiddleware)
        .data(DataLoader::new(
            ApneaSessionLoader::new(pooled_database.clone()),
            rt::spawn,
        ))
        .data(DataLoader::new(
            DiveLoaderBySession::new(pooled_database.clone()),
            rt::spawn,
        ))
        .data(DataLoader::new(
            DiveLoaderByUser::new(pooled_database.clone()),
            rt::spawn,
        ))
        // .data(DataLoader::new(
        //     ReportLoader::new(pooled_database.clone()),
        //     rt::spawn,
        // ))
        .data(DataLoader::new(
            FormLoader::new(pooled_database.clone()),
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
