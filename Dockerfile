# syntax=docker/dockerfile:1.6
# COPY-only runtime image. Build the binary and stage build/image/ locally or in CI;
# this file only copies those artifacts into gcr.io/distroless/cc-debian13:nonroot.
FROM gcr.io/distroless/cc-debian13:nonroot@sha256:d3cda6e91129130d7229a1806b6a73d292ef245ab032da7851907798024cefba

WORKDIR /app

COPY --chmod=555 sigmatactical-org /app/sigmatactical-org
COPY --chown=nonroot:nonroot static ./static

USER nonroot:nonroot

ENV PORT=8080
EXPOSE 8080

ENTRYPOINT ["/app/sigmatactical-org"]
