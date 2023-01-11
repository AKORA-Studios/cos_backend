FROM rust:1.66-alpine3.16 as builder
WORKDIR /usr/src/myapp

RUN apk add --no-cache postgresql-client libc-dev=0.7.2-r3

COPY . .
RUN cargo install --path ./api

FROM alpine3.16
RUN apk add --no-cache postgresql-client
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api

EXPOSE 8000
CMD ["api"]
