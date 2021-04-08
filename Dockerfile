# Use lastest Rust stable release as base image
FROM rust:1.49

# Switch working directory to 'app' (equivalent to 'cd app')
# If 'app' folder does not exist, Docker will create
WORKDIR app
# Copy all files from working environment to Docker image
COPY . .
ENV SQLX_OFFLINE true
# Build binary
RUN cargo build --release
# When 'docker run' is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod"]