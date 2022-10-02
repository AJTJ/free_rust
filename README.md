TODO
- Create tag system or some other system for different events
- Update to Actix-Web 4
- add better logging

NOTES:
- http://localhost:8080/ for playground

run database (from greenfield)
`docker run --name free-rust-postgres -e POSTGRES_PASSWORD=mysecretpassword -dp 5432:5432 postgres`
- ensure you have postgres installed on your machine (diesel-cli requires)
- install diesel-cli `cargo install diesel_cli --no-default-features --features postgres`
- `echo DATABASE_URL=postgres://postgres:mysecretpassword@localhost/free-rust-postgres > .env`
- `diesel setup` to set up database
- run `diesel migration run` while in the repo to build database
- start/compile the server with `cargo run`

// excellent resource
https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/graphql.rs

// diesel example
https://github.com/diesel-rs/diesel/tree/master/examples/postgres/advanced-blog-cli/migrations


Stack
- actix-web
  - https://actix.rs/docs/
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