FROM rust:1.74.0

WORKDIR /app

COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
RUN ["cargo", "build", "--release"]

CMD ["target/release/advent_calendar_backend"]
