FROM lukemathwalker/cargo-chef:latest-rust-1.75.0-slim-bullseye as chef
WORKDIR /ecs_helpers

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /ecs_helpers/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM alpine:3.19.0
COPY --from=builder /ecs_helpers/target/release/ecs_helpers .

CMD ["./ecs_helpers"]
