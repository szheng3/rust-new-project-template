# Use a Rust base image
FROM rust:latest

# Update the package repository and install dependencies
RUN apt-get update && \
    apt-get install -y software-properties-common python3-dev python3-pip libopenblas-dev libopenmpi-dev

## Add the PyTorch repository
#RUN add-apt-repository ppa:ubuntu-toolchain-r/test

# Update the package repository and install PyTorch
RUN apt-get update && \
    apt-get install -y python3-torch

# Set the working directory
WORKDIR /app

# Copy the application code
COPY . .

# Build the application
RUN cargo build --release

# Expose the application port
EXPOSE 8080

# Set the command to run when the container starts
CMD ["./target/release/<app-binary-name>"]