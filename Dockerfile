FROM rust as builder
COPY . /app
WORKDIR /app
RUN cargo build --release --verbose 
FROM gcr.io/distroless/cc-debian11
COPY --from=builder /app/target/release/starter /app/starter
COPY --from=builder /app/receipt /app/receipt
WORKDIR /app
CMD ["./starter"]