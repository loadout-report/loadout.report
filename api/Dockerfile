# Build Stage
FROM rust:1.67-slim-bullseye AS builder

WORKDIR /usr/src/
RUN rustup toolchain install nightly
# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/loadout.report
COPY . .
RUN cargo build --release --package api  # --target x86_64-unknown-linux-musl

# Bundle Stage
FROM alpine
COPY --from=builder /usr/src/loadout.report/target/x86_64-unknown-linux-musl/release/api .
USER 1000
CMD ["/api"]