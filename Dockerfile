FROM rust:1.67.1 AS build

WORKDIR /app
COPY . .

RUN cargo build --release --bin eclipse-chain-registry

FROM gcr.io/distroless/base-debian11
COPY --from=build /app/target/release/eclipse-chain-registry .
EXPOSE 8000

ENTRYPOINT ["./eclipse-chain-registry"]
