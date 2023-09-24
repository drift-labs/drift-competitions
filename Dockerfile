# syntax=docker/dockerfile:1.4
FROM switchboardlabs/sgx-function AS builder

WORKDIR /home/root/switchboard-function
COPY ./switchboard-function/Cargo.toml ./switchboard-function/Cargo.lock ./
COPY ./switchboard-function/src ./src/

RUN cargo build --release && \
    cargo strip && \
    mv target/release/simple-randomness-function /sgx/app

FROM switchboardlabs/sgx-function

# Copy the binary
WORKDIR /sgx
COPY --from=builder /sgx/app /sgx

# Get the measurement from the enclave
RUN rm -f /measurement.txt
RUN /get_measurement.sh && \
    cat /measurement.txt

ENTRYPOINT ["bash", "/boot.sh"]