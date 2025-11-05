# FROM docker.io/rust:1 as base
#
# RUN set -eux ;\
#     rustup install stable && rustup toolchain install nightly --component rust-src ;\
#     cargo install bpf-linker
#
# FROM base as build-env
#
# WORKDIR /app
#
# COPY . /app
#
# # cargo xtask build --release
# RUN cargo build --release

# Strip debugging symbols to reduce binary size
# RUN strip target/release/nflux
#
# FROM gcr.io/distroless/static-debian13:debug as release
#
# COPY --from=build-env /app/target/release/nflux /app/nflux
#
# CMD ["/app/nflux"]
FROM docker.io/library/debian:bookworm-slim

COPY ./target/release/nflux /usr/local/bin/nflux

CMD ["/usr/local/bin/nflux"]
