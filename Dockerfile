FROM rust:latest

ARG DATABASE_URL

ENV DATABASE_URL=${DATABASE_URL}

RUN apt-get update && apt-get install -y libmysqlclient-dev

WORKDIR /auto_track-backend

COPY . .

RUN cargo build --release

CMD ["./target/release/auto_track-backend"]