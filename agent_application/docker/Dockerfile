# https://github.com/LukeMathWalker/cargo-chef

FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG UNICORE__SECRET_MANAGER__STRONGHOLD__PATH
ENV UNICORE__SECRET_MANAGER__STRONGHOLD__PATH=$UNICORE__SECRET_MANAGER__STRONGHOLD__PATH
ARG UNICORE__SECRET_MANAGER__STRONGHOLD__PASSWORD
ENV UNICORE__SECRET_MANAGER__STRONGHOLD__PASSWORD=$UNICORE__SECRET_MANAGER__STRONGHOLD__PASSWORD
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin agent_application
RUN cargo build --release --bin agent_secret_manager

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/agent_application /usr/local/bin/
COPY --from=builder /app/target/release/agent_secret_manager /usr/local/bin/

COPY ../../agent_verification/presentation_definitions agent_verification/presentation_definitions

CMD ["/usr/local/bin/agent_application"]
