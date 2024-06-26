FROM rust:latest

ARG DATABASE_URL

ENV DATABASE_URL=${DATABASE_URL}
ENV SQLX_OFFLINE=true

RUN apt-get update && apt-get install -y \
    build-essential \
    default-libmysqlclient-dev

WORKDIR /auto_track-backend

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
RUN cargo install sqlx-cli

COPY . .

RUN cargo build --release

CMD ["./target/release/auto_track-backend"]