FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin rust-fubini
WORKDIR /rust-fubini
COPY . .
RUN cargo +nightly build --release

FROM debian:buster-slim
COPY --from=builder /rust-fubini/target/release/fubini /rust-fubini

ENTRYPOINT ["./rust-fubini"]
CMD ["5"]
