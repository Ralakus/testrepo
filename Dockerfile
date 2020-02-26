FROM rust:latest

COPY src /connexus/src

COPY Cargo.toml /connexus/Cargo.toml

EXPOSE 8080

RUN cargo install --path /connexus

CMD [ "connexus_server" ]