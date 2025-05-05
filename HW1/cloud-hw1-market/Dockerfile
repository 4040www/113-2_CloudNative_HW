# ===== 第一階段：Build Rust 專案 =====
FROM rust:1.81 AS builder

WORKDIR /app

COPY . .

RUN cargo generate-lockfile

RUN cargo build --release

# ===== 第二階段：用相容的新版本 Debian =====
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y bash && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/cloud-hw1-market ./cloud-hw1-market
COPY nonexistent.sh .

RUN chmod +x ./cloud-hw1-market ./run.sh

CMD ["./run.sh"]
