# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Set the command to run the application
CMD ["./src/main"]