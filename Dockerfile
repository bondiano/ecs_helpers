FROM  lukemathwalker/cargo-chef:latest-rust-1.75.0-alpine as chef
WORKDIR /ecs_helpers

FROM --platform=$BUILDPLATFORM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

RUN apk update && apk upgrade && \
  apk add libressl-dev
COPY --from=planner /ecs_helpers/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM --platform=$BUILDPLATFORM docker:24.0.7-cli-alpine3.19
WORKDIR /app

COPY --from=builder /ecs_helpers/target/release/ecs_helpers /

RUN alias ecs_helper="/ecs_helpers"
