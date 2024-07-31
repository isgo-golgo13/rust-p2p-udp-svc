# Build stage
FROM debian:buster AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /usr/src/p2p_svc

# Copy the project files
COPY . .

# Build the project in release mode
RUN cargo build --release --manifest-path p2p/Cargo.toml

# Verify the build output directory structure
RUN find /usr/src/p2p_svc -type d -name "release"
RUN find /usr/src/p2p_svc -type f -name "p2p"

# Final stage
FROM debian:buster-slim

# Create a non-root user
RUN useradd -ms /bin/bash appuser

# Set the working directory
WORKDIR /home/appuser

# Copy the built binary from the build stage
COPY --from=builder /usr/src/p2p_svc/p2p/target/release/p2p .

# Ensure the binary is executable
RUN chmod +x p2p

# Change ownership of the binary to the non-root user
RUN chown appuser:appuser p2p

# Switch to the non-root user
USER appuser

# Expose the necessary port (example: 12345)
EXPOSE 12345

# Run the application
CMD ["./p2p"]
