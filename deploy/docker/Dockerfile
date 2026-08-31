FROM rust:1.80-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p glyph-cli

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/glyph /usr/local/bin/glyph
CMD ["glyph", "version"]
