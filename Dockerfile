# Build stage: Install build dependencies and build api_reverse_proxy
FROM rust:1.75.0 as build-api_reverse_proxy
RUN USER=root cargo new --bin api
WORKDIR /api
RUN mkdir -p src/api && mv src/main.rs src/api/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --bin api
RUN rm -rf src/*
COPY src ./src
RUN rm ./target/release/deps/api*
RUN cargo build --release --bin api
RUN cargo build --release --bin authorize_key

# Build stage: Install build dependencies and build diesel_cli
FROM rust:1.75.0-slim-buster as build-db_push_reverse_proxy
RUN apt-get update && \
    apt-get install -y libpq-dev pkg-config && \
    cargo install diesel_cli --no-default-features --features "postgres"

# api_reverse_proxy image
FROM debian:12.0-slim as api_reverse_proxy
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=build-api_reverse_proxy /api/target/release/api /usr/local/bin/api
CMD ["/usr/local/bin/api"]

# db_push_reverse_proxy image
FROM debian:buster-slim as db_push_reverse_proxy
COPY --from=build-db_push_reverse_proxy /usr/local/cargo/bin/diesel /usr/local/bin/
RUN apt-get update && \
    apt-get install -y libpq5 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
WORKDIR /diesel
COPY diesel.toml ./
COPY migrations ./migrations
CMD ["diesel", "migration", "run"]

# ssh_reverse_proxy image
FROM debian:12.0-slim as ssh_reverse_proxy
RUN apt-get update && apt-get install -y openssh-server libpq-dev supervisor && apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN echo 'root:root' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitUserEnvironment no/PermitUserEnvironment yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitTunnel no/PermitTunnel yes/' /etc/ssh/sshd_config
RUN sed -i 's/#GatewayPorts no/GatewayPorts yes/' /etc/ssh/sshd_config

RUN mkdir -p /run/sshd
RUN mkdir -p /var/log/supervisor
RUN touch /root/.ssh/authorized_keys

COPY --from=build-api_reverse_proxy /api/target/release/authorize_key /usr/local/bin/authorize_key
COPY services/authorize_key.conf /etc/supervisor/conf.d/supervisord.conf

EXPOSE 22
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
