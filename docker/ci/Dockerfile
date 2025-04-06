FROM cgr.dev/chainguard/glibc-dynamic:latest AS release

ARG TARGET_BIN
ENV OPERATOR_BIN=$TARGET_BIN

USER root

COPY --chmod=755 bin/${TARGET_BIN} /${TARGET_BIN}

ENTRYPOINT ["/${OPERATOR_BIN}"]
