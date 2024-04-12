FROM rust:1.75.0 as base
ARG DATABASE_URL=app.db
WORKDIR /app
ADD . /app
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel migration run
RUN cargo build --release --bin api

FROM debian:12.0-slim as runtime
ARG DATABASE_URL=app.db
WORKDIR /app
COPY --from=base /app/target/release/api /app
COPY --from=base /app/$DATABASE_URL /app
EXPOSE 8081
ENTRYPOINT ["./api"]

FROM debian:12 as ssh_reverse_proxy

RUN apt-get update && apt-get install -y openssh-server
RUN echo 'root:root' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitEmptyPasswords no/PermitEmptyPasswords yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitUserEnvironment no/PermitUserEnvironment yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitTunnel no/PermitTunnel yes/' /etc/ssh/sshd_config


RUN mkdir -p /run/sshd

EXPOSE 22

CMD ["/usr/sbin/sshd", "-D"]
