ARG APP_NAME

FROM alpine:latest
ENV APP_NAME_ENV=${APP_NAME}
RUN echo "1. Variable with double quotes around all APP_NAME    =${APP_NAME} - did you see that?"
RUN echo "1. Variable with double quotes around all APP_NAME_ENV=${APP_NAME_ENV} - did you see that?"
RUN echo '2. Variable with single quotes around all APP_NAME    =${APP_NAME} - did you see that?'
RUN echo '2. Variable with single quotes around all APP_NAME_ENV=${APP_NAME_ENV} - did you see that?'
RUN echo "3. Variable with double quotes around     APP_NAME    ="${APP_NAME}" - did you see that?"
RUN echo "3. Variable with double quotes around     APP_NAME_ENV="${APP_NAME_ENV}" - did you see that?"
RUN echo '4. Variable with single quotes around     APP_NAME    ='${APP_NAME}' - did you see that?'
RUN echo '4. Variable with single quotes around     APP_NAME_ENV='${APP_NAME_ENV}' - did you see that?'
RUN echo 5. Variable without quotes                 APP_NAME    =${APP_NAME} - did you see that?
RUN echo 5. Variable without quotes                 APP_NAME_ENV=${APP_NAME_ENV} - did you see that?
RUN echo 6. Variable with double quotes             APP_NAME    ="${APP_NAME}" - did you see that?
RUN echo 6. Variable with double quotes             APP_NAME_ENV="${APP_NAME_ENV}" - did you see that?
RUN echo 7. Variable with single quotes             APP_NAME    ='${APP_NAME}' - did you see that?
RUN echo 7. Variable with single quotes             APP_NAME_ENV='${APP_NAME_ENV}' - did you see that?
COPY docker-entrypoint.sh /
CMD ["/docker-entrypoint.sh"]

# Ignore the rest... 

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
# FROM rust:latest as cargo-build
# RUN apt-get update
# RUN apt-get install musl-tools -y
# RUN rustup target add x86_64-unknown-linux-musl
# WORKDIR /usr/src/app
# COPY Cargo.toml Cargo.toml
# RUN mkdir src/
# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/${APP_NAME_ARG}*
# COPY . .
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# # ------------------------------------------------------------------------------
# # Final Stage
# # ------------------------------------------------------------------------------
# FROM alpine:latest
# ENV APP_NAME_ENV=${APP_NAME_ARG}
# RUN echo "This should be a text: ${APP_NAME_ARG}"
# RUN addgroup -g 1000 app
# RUN adduser -D -s /bin/sh -u 1000 -G "app" "app"
# WORKDIR /home/app/bin/
# COPY --from=cargo-build "/usr/src/app/target/x86_64-unknown-linux-musl/release/${APP_NAME_ARG}" .
# RUN chown "app:app" "${APP_NAME_ARG}"
# USER "app"
# CMD ["./${APP_NAME_ARG}"]
