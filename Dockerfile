FROM rust:latest as cargo-build
ARG APP_NAME
RUN apt-get update
RUN apt-get install musl-tools -y
WORKDIR /app
