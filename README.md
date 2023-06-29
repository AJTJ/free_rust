
## CURRENT
- Code -> Free Source



## Dev Env
- Start DB image
- `cargo run`
- http://localhost:8080/ for graphql playground

## Diesel commands
- diesel migration revert
- diesel migration run


## how to set up redis (from greenfield)
`docker run --name free-redis -dp 6379:6379 redis redis-server --save 60 1 --loglevel warning`



## how to run database (from greenfield)
`docker run --name free-rust-postgres -e POSTGRES_PASSWORD=mysecretpassword -dp 5432:5432 postgres`
- ensure you have postgres installed on your machine (diesel-cli requires)
- install diesel-cli `cargo install diesel_cli --no-default-features --features postgres`

## Once server image is loaded
- `diesel setup` to set up database
- run `diesel migration run` while in the repo to build database
- start/compile the server with `cargo run`

// excellent resource
https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/graphql.rs

// diesel example
https://github.com/diesel-rs/diesel/tree/master/examples/postgres/advanced-blog-cli/migrations


### gql things
- for partially fallible user-defined-resolvers
  - https://github.com/async-graphql/async-graphql/issues/531
- Notes on simple object macros
  - https://docs.rs/async-graphql/latest/async_graphql/derive.SimpleObject.html

## The Proposed Stack
- actix-web
  - https://actix.rs/docs/
  - https://github.com/actix/examples/tree/master/graphql/async-graphql
- user auth
  - JWT
    - libs
      - https://github.com/michaelvanstraten/actix-jwt-auth-middleware
    - tutorials
      - https://docs.rs/actix-jwt-auth-middleware/latest/actix_jwt_auth_middleware/
      - https://formidable.com/blog/2022/authn-and-authz/
      - https://www.apollographql.com/docs/apollo-server/security/authentication/
      - https://www.section.io/engineering-education/how-to-build-authentication-api-with-jwt-token-in-nodejs/
      - https://jwt.io/introduction
      - https://reactnavigation.org/docs/auth-flow/
- database postgres
  - diesel
    - https://diesel.rs/
  - bb8 for thread pool (as opposed to r2d2)
    - https://docs.rs/bb8/latest/bb8/
    - https://github.com/djc/bb8
- async-graphql for front-end interaction
  - https://www.apollographql.com/docs/kotlin/tutorial/03-write-your-first-query/
  - https://github.com/async-graphql/async-graphql
  - https://async-graphql.github.io/async-graphql/en/index.html
  - https://docs.rs/async-graphql-actix-web/3.0.27/async_graphql_actix_web/
  - example
    - https://github.com/phated/twentyfive-stars
- auth
  - rust-argon2
- logging
  - tracing
    - https://crates.io/crates/tracing

FUTURE THOUGHTS
- database
  - geospatial storage: https://ridewithgps.com/careers/systems_engineer