# Use an official Rust image as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /usr/src/app

# Install sqlx-cli first to take advantage of Docker's cache layering
# sqlx-cli is installed globally and only needs to be reinstalled if the version changes
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Copy the current directory contents into the container at /usr/src/app
# Copy the manifests first to cache your dependencies
COPY Cargo.toml Cargo.lock ./

# This dummy build step will cache your dependencies
# It will only re-run if your dependencies change
RUN mkdir src/ && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/ && \
    rm target/release/deps/rust_blog*

# Now copy your actual source code
COPY . .

# Build your program for production
# RUN cargo build --release

# Copy the built binary to a different stage to reduce image size
# FROM ubuntu:latest
# COPY --from=0 /usr/src/app/target/release/rust-blog /usr/local/bin/rust-blog

# # The default command runs the binary
# CMD ["rust-blog"]