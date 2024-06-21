FROM cgr.dev/chainguard/glibc-dynamic:latest-dev as dev-local

ARG TARGET_BIN
ENV OPERATOR_BIN $TARGET_BIN

USER root

COPY bin/${TARGET_BIN} /${TARGET_BIN}

RUN chmod +x /${TARGET_BIN}

ENTRYPOINT /$OPERATOR_BIN
