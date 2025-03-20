FROM docker.io/rust:1 as base

RUN set -eux ;\
    rustup install stable && rustup toolchain install nightly --component rust-src ;\
    cargo install bpf-linker

FROM base as build-env

WORKDIR /app

COPY . /app

# cargo xtask build --release
RUN cargo build --release

# Strip debugging symbols to reduce binary size
RUN strip target/release/nflux

FROM gcr.io/distroless/cc-debian12 as release

COPY --from=build-env /app/target/release/nflux /app/nflux

CMD ["/app/nflux"]
