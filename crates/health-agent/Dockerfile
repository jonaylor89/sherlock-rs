# Stage 1: Build
FROM rust:1-slim-bookworm as build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo build --release -p sherlock-health-agent

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/sherlock-health-agent /usr/local/bin/sherlock-health-agent

VOLUME /data

ENTRYPOINT ["sherlock-health-agent"]
CMD ["--local", "--output", "/data/health_report.json"]
