ARG  TAG

FROM rust:${TAG}

RUN apt-get update && apt-get -y install libssl-dev build-essential musl-dev libc6

WORKDIR /app