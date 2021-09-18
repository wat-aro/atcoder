FROM rust:1.53.0-buster AS cargo-generate
RUN apt-get update -qq \
  && apt-get install libssl-dev pkg-config
RUN cargo install cargo-generate

FROM rust:1.42.0-buster

ENV LANG=ja_JP.UTF-8 \
    TZ=Asia/Tokyo \
    USER=root \
    APP_HOME=/usr/src/app

WORKDIR $APP_HOME

RUN rustup default 1.42.0 \
  && rustup component add rustfmt \
  && rustup component add rust-src

COPY --from=cargo-generate /usr/local/cargo/bin/cargo-generate /usr/local/cargo/bin/cargo-generate
