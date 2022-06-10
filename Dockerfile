# # BUILD
# FROM rust:1.61 as builder

# WORKDIR /usr/src/sema-api

# COPY . .

# RUN apt-get update && apt-get -y install link-grammar libclang-dev

# RUN cargo build --release


# # RUN
# FROM debian:buster-slim

# RUN apt-get update && apt-get -y install openssl ca-certificates link-grammar

# WORKDIR /usr/local/bin/sema-api

# COPY --from=builder /usr/src/sema-api/target/release/sema-api ./

# EXPOSE 8088
# EXPOSE 443

# CMD ["./sema-api"]

FROM ubuntu:20.04

COPY . .

# Set tz to prevent warnings while install dependencies
RUN ln -snf /usr/share/zoneinfo/$CONTAINER_TIMEZONE /etc/localtime && echo $CONTAINER_TIMEZONE > /etc/timezone

RUN apt-get update && DEBIAN_FRONTEND=noninteractive && apt-get install -y --no-install-recommends \
  link-grammar \
  build-essential \
  curl \
  ca-certificates \
  pkg-config \
  openssl \
  libssl-dev \
  libclang-dev \
  flex

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# install link-grammar, so that the rust compiler can bind against it.
RUN chmod u+r+x install_link_grammar.sh
RUN ./install_link_grammar.sh


WORKDIR /sema-api
RUN cargo build --release

WORKDIR /sema-api/target/release

EXPOSE 8088
EXPOSE 443

CMD ["./sema-api"]