FROM rust:1.75.0 as build-api_reverse_proxy
RUN USER=root cargo new --bin api
WORKDIR /api
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --bin api
RUN rm src/*.rs
COPY src ./src
RUN rm ./target/release/deps/api*
RUN cargo build --release --bin api

# api_reverse_proxy image
FROM debian:12.0-slim as api_reverse_proxy
COPY --from=build-api_reverse_proxy /api/target/release/api /usr/local/bin/api
CMD ["/usr/local/bin/api"]

# Use a lighter Rust image for building
FROM rust:1.75.0 as build-db_push_reverse_proxy
RUN cargo install diesel_cli --no-default-features --features postgres

# Use a minimal base image for the runtime
FROM debian:buster-slim as db_push_reverse_proxy
COPY --from=build-db_push_reverse_proxy /usr/local/cargo/bin/diesel /usr/local/bin/
WORKDIR /diesel
COPY diesel.toml ./
COPY migrations ./migrations
CMD ["diesel", "migration", "run"]

# ssh_reverse_proxy image
FROM debian:12 as ssh_reverse_proxy
RUN apt-get update && apt-get install -y openssh-server
RUN echo 'root:root' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitEmptyPasswords no/PermitEmptyPasswords yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitUserEnvironment no/PermitUserEnvironment yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitTunnel no/PermitTunnel yes/' /etc/ssh/sshd_config
RUN sed -i 's/#GatewayPorts no/GatewayPorts yes/' /etc/ssh/sshd_config
RUN mkdir -p /run/sshd

EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]
