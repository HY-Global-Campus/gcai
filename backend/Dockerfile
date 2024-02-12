FROM rust:1.76 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/gcai-backend
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim


# Install build dependencies
RUN apt update && apt install -y build-essential wget libssl-dev && rm -rf /var/lib/apt/lists/*

# Download and compile OpenSSL 3.0
RUN wget https://www.openssl.org/source/openssl-3.0.0.tar.gz \
    && tar -zxf openssl-3.0.0.tar.gz \
    && cd openssl-3.0.0 \
    && ./config \
    && make \
    && make install

# Update shared library cache
RUN ldconfig


COPY --from=build /usr/src/gcai-backend/target/release/gcai-backend /usr/local/bin/gcai-backend

CMD ["gcai-backend"]

