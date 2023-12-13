# Use an official Rust image as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the current directory contents into the container
COPY . .

# Sqlx setup
RUN cargo install sqlx-cli --no-default-features --features sqlite

RUN sqlx database create --database-url sqlite:data.db

# Build your program for production
RUN cargo install --path .

# Run the binary
CMD ["rust-blog"]