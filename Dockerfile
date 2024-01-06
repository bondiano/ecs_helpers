FROM lukemathwalker/cargo-chef:latest-rust-1.75.0-slim-bullseye as chef
WORKDIR /ecs_helpers

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

RUN apt-get update && \
    apt-get -y install pkg-config openssl libssl-dev
COPY --from=planner /ecs_helpers/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

RUN apt-get update && \
  apt-get install -y ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /ecs_helpers/target/release/ecs_helpers /

ENTRYPOINT ["/ecs_helpers"]
