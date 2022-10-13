# THIS IS USE FOR BUILDING THE IMAGE EXAMPLE PROJECT ACTIX V8
################
##### Builder
FROM rust:1.61.0-slim as builder

WORKDIR /usr/src

# Create blank project

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/ssr_actix_v8/

# Now copy in the rest of the sources
COPY . /usr/src/ssr_actix_v8

# Set the working directory
WORKDIR /usr/src/ssr_actix_v8

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is the actual application build.
RUN cargo build --example actix-v8 --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime 

WORKDIR /app

# Copy application binary from builder image
COPY --from=builder /usr/src/ssr_actix_v8/target/x86_64-unknown-linux-musl/release/examples/actix-v8 /app

# Create a example web to run the application
RUN mkdir -p examples/simple-ssr/source/dist

COPY --from=builder /usr/src/ssr_actix_v8/examples/simple-ssr/source/dist /app/examples/simple-ssr/source/dist

# exposing out base on server PORT
EXPOSE 8082

# Run the application
CMD ["/app/actix-v8"]