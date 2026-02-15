# Release instructions:
# 1. Update the version tag in the Dockerfile to match the version in Cargo.toml
# 2. Update the VCS_REF tag to match the tagged version's FULL commit hash
# 3. Build image with BOTH latest and version tags
#    i.e. `docker build -t sherlock/sherlock:0.1.0 -t sherlock/sherlock:latest .`

# Stage 1: Build the Rust application
FROM rust:1-slim-bookworm as build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo build --release -p sherlock

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim
WORKDIR /sherlock

ARG VCS_REF= # CHANGE ME ON UPDATE
ARG VCS_URL="https://github.com/jonaylor89/sherlock-rs"
ARG VERSION_TAG= # CHANGE ME ON UPDATE

LABEL org.label-schema.vcs-ref=$VCS_REF \
    org.label-schema.vcs-url=$VCS_URL \
    org.label-schema.name="Sherlock" \
    org.label-schema.version=$VERSION_TAG \
    website="https://sherlockproject.xyz"

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the build stage
COPY --from=build /app/target/release/sherlock /usr/local/bin/sherlock

# Specify the entrypoint of the container
ENTRYPOINT ["sherlock"]
