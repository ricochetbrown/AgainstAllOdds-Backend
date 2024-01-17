# Use a specific Rust image as the base
FROM rust:latest

# Set the working directory within the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml ../../Cargo.lock ./

# Build the Rust application
RUN cargo build --release

# Copy the Rust source code to the container
COPY src ./src

# Build the application again to make sure all dependencies are built
RUN cargo build --release

# Set the binary executable as the entry point
ENTRYPOINT ["./target/release/rust-app"]