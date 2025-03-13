# syntax=docker/dockerfile:1
#
# First Stage: Build the application using the official Rust image.
#
FROM rust:latest AS builder

# Create and switch to the application directory.
WORKDIR /usr/src/app

# Cache dependencies:
# Copy Cargo.toml and Cargo.lock to cache Rust dependencies.
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to allow dependency resolution before copying the full source.
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build an empty project; this caches the dependencies.
RUN cargo build --release

# Now copy the actual source code. This will invalidate the cache only when source files change.
COPY . .

# Build the application in release mode.
RUN cargo build --release

#
# Second Stage: Create a minimal runtime image.
#
FROM debian:buster-slim

# Set the working directory for the final image.
WORKDIR /usr/local/bin

# Install required certificates (adjust if your app requires network communication).
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage.
# Replace "rtree" below with the name of your final binary if different.
COPY --from=builder /usr/src/app/target/release/rtree .


# Set the command to run the binary.
CMD ["./rtree"]