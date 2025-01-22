FROM docker.io/rust:1 as build-env

WORKDIR /app

RUN set -eux ;\
    rustup install stable && rustup toolchain install nightly --component rust-src ;\
    cargo install bpf-linker

COPY . /app

# cargo xtask build --release
RUN cargo build --release

# Strip debugging symbols to reduce binary size
RUN strip target/release/nflux

FROM gcr.io/distroless/cc-debian12

COPY --from=build-env /app/target/release/nflux /app/nflux

CMD ["/app/nflux"]
