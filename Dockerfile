FROM rust:1.66-alpine3.16 as builder
WORKDIR /usr/src/myapp

# Needed for diesel
RUN apk add --no-cache postgresql-client libc-dev=0.7.2-r3

# Create the same scaffhold to build and cache dependecies
RUN cargo new --bin api && \
    cargo new --lib application && \
    cargo new --lib domain && \
    cargo new --lib infrastructure && \
    cargo new --lib shared

COPY ./Cargo.toml ./Cargo.toml
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./application/Cargo.toml ./application/Cargo.toml
COPY ./domain/Cargo.toml ./domain/Cargo.toml
COPY ./infrastructure/Cargo.toml ./infrastructure/Cargo.toml
COPY ./shared/Cargo.toml ./shared/Cargo.toml

# Build regardles of any code changes just to cache deps
# This will only rebuild if any Cargo.toml file was changed
# but not if actualy code was changed 
RUN cargo build --workspace --release

# Clean up
RUN rm -rf api application domain infrastructure shared
RUN rm ./target/release/*.d ./target/release/*.rlib ./target/release/api
# Copy the actual code
COPY . .

# Build with precached dependencies
RUN cargo install --locked --path ./api



FROM alpine3.16 as runner
RUN apk add --no-cache postgresql-client
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api

RUN echo DATABASE_URL=postgres://cos:password@db/cos > .env

EXPOSE 8000
CMD ["api"]
