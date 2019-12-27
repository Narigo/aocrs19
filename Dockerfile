FROM rust:latest as cargo-build
ARG APP_NAME
RUN apt-get update
RUN apt-get install musl-tools -y
WORKDIR /app
# RUN rustup target add x86_64-unknown-linux-musl
# RUN rustup target add x86_64-apple-darwin
CMD ["cargo", "build", "--release"]
