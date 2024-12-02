FROM alpine:3.20

ARG  TAG_VERSION

RUN apk add --no-cache \
        ca-certificates \
        gcc

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=$TAG_VERSION

COPY scripts/rustup.sh /opt/scripts/rustup.sh
RUN /opt/scripts/rustup.sh

WORKDIR /app
