### gasket ci image: Pushed to pierrelefevreneti/gasket by GitHub Actions

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

## Stage 2: Setup the runtime environment
FROM ubuntu as runtime

# Install necessary runtime dependencies
RUN apt-get update && apt-get upgrade -y && apt-get autoremove -y 
RUN apt-get install wget ffmpeg -y && rm -rf /var/lib/apt/lists/*

# Copy the binary and any other necessary files from the builder stage
COPY --from=builder /gasket/target/release/gasket /usr/local/bin/gasket

# Get data folder with example videos
RUN mkdir /data
WORKDIR /data
ENV DATA_DIR=/data
# RUN wget https://gsktfs.app.kista.cloud/test_loop.mp4
COPY ./data/test_loop.mp4 test_loop.mp4

# Prepare for takeoff
WORKDIR /
EXPOSE 30303/udp
EXPOSE 8080/tcp
ENV RUST_LOG=info

# Run the application
CMD gasket