FROM rust:latest as cargo-build
ARG APP_NAME
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
CMD ["cargo", "build", "--release"]
