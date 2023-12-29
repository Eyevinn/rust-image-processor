# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the entire project to the working directory
COPY . .

# Build the project
RUN cargo build --release

# Set the startup command for the container
CMD ["./target/release/rust-image-processor"]
