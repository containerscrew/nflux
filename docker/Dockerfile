FROM docker.io/rust:1 as build-env

WORKDIR /app

COPY . /app

RUN set -eux ;\
    cargo install bpf-linker ;\
    rustup install stable && rustup toolchain install nightly --component rust-src

# cargo xtask build --release
RUN cargo xtask build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build-env /app/target/release/nflux /

CMD ["./nflux"]
