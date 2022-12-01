FROM rust:1.50 as builder
WORKDIR /usr/src/myapp
COPY . .
#RUN cargo install --path .
RUN cargo build --release

FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/myapp/target/release/ActixwebTrigger ./

EXPOSE 8081
CMD ["./ActixwebTrigger"]
