FROM rust:1.52 as builder
WORKDIR /opt
COPY . .
RUN cargo install --path .

FROM alpine:latest
WORKDIR /opt
COPY --from=builder /usr/local/cargo/bin/korean-it-recruitment-alert /usr/local/bin/korean-it-recruitment-alert
CMD [ "korean-it-recruitment-alert" ]
