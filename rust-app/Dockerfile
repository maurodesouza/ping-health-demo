FROM rust:1.88.0-alpine3.22

RUN apk add --no-cache musl-dev pkgconfig openssl-dev bash curl git

WORKDIR /app

RUN cargo install --locked watchexec-cli

COPY . .

CMD ["watchexec", "-r", "-i", "target/**", "cargo", "run"]

EXPOSE 9090
