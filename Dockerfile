FROM rust:latest AS builder
WORKDIR app
COPY . .
ENV SQL_OFFLINE true
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update; \
    apt-get install -y --no-install-recommends openssl; \
    rm -rf /var/lib/apt/lists/*; \
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY conf conf
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]