### gasket-lb image: Pushed to pierrelefevreneti/gasket-lb by GitHub Actions

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
RUN USER=root cargo new --bin gasket-lb
WORKDIR /gasket-lb

COPY ./ ./

# Build the application
RUN cargo build --release

## Stage 2: Setup the runtime environment
FROM ubuntu as runtime

# Copy the binary and any other necessary files from the builder stage
COPY --from=builder /gasket-lb/target/release/gasket-lb /usr/local/bin/gasket-lb

# Get latest dependencies
RUN apt-get update && apt-get upgrade -y && apt-get autoremove -y 

# Prepare for takeoff
WORKDIR /
EXPOSE 8080/tcp
ENV RUST_LOG=info

# Run the application
CMD gasket-lb