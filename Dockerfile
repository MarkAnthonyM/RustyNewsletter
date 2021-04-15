# FROM lukemathwalker/cargo-chef as planner
# WORKDIR app
# COPY . .
# Compute a lock-like file for our project
# RUN cargo chef prepare --recipe-path recipe.json

# FROM lukemathwalker/cargo-chef as cacher
# WORKDIR app
# COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application
# RUN cargo chef cook --release --recipe-path recipe.json

# Use lastest Rust stable release as base image
FROM rust AS builder

# Switch working directory to 'app' (equivalent to 'cd app')
# If 'app' folder does not exist, Docker will create
WORKDIR app
# Copy over the cached dependencies
# COPY --from=cacher /app/target target
# COPY --from=cacher /usr/local/cargo /usr/local/cargo
# Copy all files from working environment to Docker image
COPY . .
ENV SQLX_OFFLINE true
# Build application, leveraging the cached dependencies
RUN cargo build --release --bin zero2prod

# Runtime stage
FROM debian:buster-slim AS runtime

WORKDIR app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production
# When 'docker run' is executed, launch the binary
ENTRYPOINT ["./zero2prod"]