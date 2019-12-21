FROM rustlang/rust:nightly-buster-slim AS builder
#
ENV target x86_64-unknown-linux-musl
ENV flags  -Clinker=musl-gcc
#
RUN apt-get update                                     && \
    apt-get install -y --no-install-recommends musl-tools
RUN rustup update          && \
    rustup target add $target
WORKDIR /build
# load and compile dependencies using dummy main.rs to facilitate incremental Docker build
ADD Cargo.toml .
RUN mkdir src; echo "fn main() {}" > src/main.rs && \
    cargo update                                 && \
    cargo check --release --target=$target       && \
    rm src/main.rs; rmdir src
# build real app
ADD src src
RUN RUSTFLAGS=$flags cargo build --release --target=$target --frozen

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
WORKDIR /app
COPY --from=builder /build/target/*/release/hello-rocket ./app
EXPOSE 8000
ENTRYPOINT ["./app"]
