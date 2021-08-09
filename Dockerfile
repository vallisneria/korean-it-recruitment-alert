FROM rust:latest as builder
LABEL org.opencontainers.image.source https://github.com/vallisneria/korean-it-recruitment-alert
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /opt
COPY --from=builder /build/target/release/korean-it-recruitment-alert /opt/korean-it-recruitment-alert
RUN apt-get update
RUN apt-get install -y openssl ca-certificates
EXPOSE 80 443
CMD [ "./korean-it-recruitment-alert" ]
