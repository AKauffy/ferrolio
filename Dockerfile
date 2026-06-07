# --- Build stage ---
FROM rust:1.78-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/portfolio-backend .
COPY posts ./posts

ENV RUST_LOG=portfolio_backend=info
ENV PORT=8080
ENV POSTS_DIR=/app/posts
# Set this to your Next.js domain in production
ENV ALLOWED_ORIGIN=https://portfolio.kauffy.dev

EXPOSE 8080
CMD ["./portfolio-backend"]
