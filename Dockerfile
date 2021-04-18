FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application! 
RUN cargo chef cook --release --recipe-path recipe.json

# Builder stage
FROM rust AS builder

WORKDIR app
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
# Build our application, leveraging the cached deps!
RUN cargo build --release --bin ecommerce-api


# Runtime stage
FROM debian:buster-slim AS runtime

WORKDIR app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment 
# to our runtime environment
COPY --from=builder /app/target/release/ecommerce-api ecommerce-api

ENTRYPOINT ["./ecommerce-api"]


