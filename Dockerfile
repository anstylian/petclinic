FROM rust:latest as builder

COPY . /app
WORKDIR /app
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel migration run
RUN cargo build --release

FROM ubuntu:latest
RUN apt update
RUN apt install -y sqlite3

WORKDIR /app
COPY --from=builder /app/db.sqlite /app/.
COPY --from=builder /app/target/release/petclinic /app/.
RUN mkdir -p /app/petclinic_config
COPY --from=builder /app/petclinic_config /app/petclinic_config
COPY templates /app/templates

ENTRYPOINT [ "/app/petclinic" ]
