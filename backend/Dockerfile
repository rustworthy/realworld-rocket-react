FROM docker.io/rust:1-slim-bookworm AS build

WORKDIR /build

COPY . .

ENV SQLX_OFFLINE=1

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/realworld-axum-react ./main

################################################################################

FROM docker.io/debian:bookworm-slim

WORKDIR /app

# copy the main binary
COPY --from=build /build/main ./

# copy runtime assets which may or may not exist
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates
COPY --from=build /build/migration[s] ./migrations

CMD ["./main"]