run database (from greenfield)
`docker run --name free-rust-postgres -e POSTGRES_PASSWORD=mysecretpassword -dp 5432:5432 postgres`
- ensure you have postgres installed on your machine (diesel-cli requires)
- install diesel-cli `cargo install diesel_cli --no-default-features --features postgres`
- `echo DATABASE_URL=postgres://postgres:mysecretpassword@localhost/free-rust-postgres > .env`
- `diesel setup` to set up database
- run `diesel migration run` while in the repo to build database
- start/compile the server with `cargo run`

TODO
- Update to Actix-Web 4
- add better logging

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