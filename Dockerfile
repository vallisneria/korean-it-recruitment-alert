FROM rust:1.52 as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /opt
COPY --from=builder /build/target/release/korean-it-recruitment-alert /opt/korean-it-recruitment-alert
CMD [ "korean-it-recruitment-alert" ]
