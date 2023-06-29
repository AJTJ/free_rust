FROM rust:buster as builder
WORKDIR /app

ENV DATABASE_URL=postgres://postgres:mysecretpassword@localhost/free-rust-postgres
ENV REDIS_URL=redis://127.0.0.1/
ENV ENVIRONMENT=PRODUCTION

COPY . .

RUN cargo build --release

FROM bitnami/minideb:buster as runner
WORKDIR /app

ENV DATABASE_URL=postgres://postgres:mysecretpassword@localhost/free-rust-postgres
ENV REDIS_URL=redis://127.0.0.1/
ENV ENVIRONMENT=PRODUCTION

COPY --from=builder /app/target/release/free_rust free_rust

CMD /app/free_rust