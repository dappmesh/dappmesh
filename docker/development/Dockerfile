FROM alpine:3.21

ARG ARG_IMAGE_NAME
ARG ARG_RUST_VERSION

RUN apk add --no-cache \
        ca-certificates \
        gcc

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=$ARG_RUST_VERSION

COPY scripts/rustup.sh /opt/scripts/rustup.sh
RUN chmod +x /opt/scripts/rustup.sh && /opt/scripts/rustup.sh

WORKDIR /app
