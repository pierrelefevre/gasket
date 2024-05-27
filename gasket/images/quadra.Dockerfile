### gasket:quadra image: Pushed to pierrelefevreneti/gasket:quadra by GitHub Actions

## Stage 1: Build the application
FROM rust:1-bookworm as builder

# Get latest dependencies
RUN apt-get update && apt-get upgrade -y && apt-get autoremove -y 

# Get release info
ARG RELEASE_BRANCH
ARG RELEASE_DATE
ARG RELEASE_COMMIT
ENV BUILD_VERSION="${RELEASE_BRANCH}-${RELEASE_DATE}-${RELEASE_COMMIT}"

# Create a new empty shell project
RUN USER=root cargo new --bin gasket
WORKDIR /gasket

COPY ./ ./

# Build the application
RUN cargo build --release

## Stage 2: Setup the runtime environment (Quadra SDK)
FROM pierrelefevreneti/quadra:sdk as runtime
USER root 

RUN apt-get update && apt-get upgrade -y && apt-get autoremove -y
RUN apt-get install build-essential libssl-dev -y && rm -rf /var/lib/apt/lists/*

# Copy the binary and any other necessary files from the builder stage
COPY --from=builder /gasket/target/release/gasket /usr/local/bin/gasket

# Get data folder with example video
RUN mkdir /data
WORKDIR /data
ENV DATA_DIR=/data
COPY ./data/test_loop.mp4 test_loop.mp4

# Prepare for takeoff
WORKDIR /
EXPOSE 30303/udp
EXPOSE 8080/tcp
ENV RUST_LOG=info

# Run the application (first ensure Quadra SDK is running)
CMD ["/bin/bash", "-c", "ni_rsrc_mon && gasket"]