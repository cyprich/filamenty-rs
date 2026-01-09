# https://eastondev.com/blog/en/posts/dev/20251217-docker-multistage-build/#rust-applications-minimal-deployment
FROM rust:1.92.0 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY app ./app
COPY lib ./lib
COPY migrations ./migrations
COPY images ./default_images

RUN cargo fetch
RUN cargo build --release

# FROM gcr.io/distroless/cc-debian12
FROM debian:12-slim
WORKDIR /app
COPY --from=builder /app/target/release/app .
COPY --from=builder /app/default_images ./default_images

COPY docker-start.sh .
RUN chmod +x docker-start.sh

CMD ["./app"]
