
# configure builder
FROM rust:1.59.0 AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
# Set SQLX to use `sqlx-data.json`, prepare allows sqlx to check metadata against this file instead of calling the db
# at compile time which fails without mitigation at the docker build stage
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
RUN cargo build --release

# configure runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production

# entrypoint
ENTRYPOINT ["./target/release/zero2prod"]


