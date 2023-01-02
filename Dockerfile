FROM rust:1.66-alpine3.16 as builder
WORKDIR /usr/src/myapp

RUN apk add --no-cache postgresql-client

COPY . .
RUN cargo install --path .

FROM alpine3.16
RUN apk add --no-cache postgresql-client
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

EXPOSE 8000
CMD ["myapp"]
