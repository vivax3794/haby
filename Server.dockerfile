FROM rust AS base
RUN cargo install cargo-chef --version ^0.1
 
FROM base AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
 
FROM base AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --package haby_server

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --package haby_server

FROM debian
WORKDIR /app

RUN apt update
RUN apt install curl -y

COPY --from=builder /app/target/release/haby_server .
COPY ./haby_server/Rocket.toml .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_LOG_LEVEL=normal

HEALTHCHECK --start-period=10s --start-interval=1s --interval=60s CMD curl --fail http://localhost:8000/version
EXPOSE 8000

CMD ["./haby_server"]
