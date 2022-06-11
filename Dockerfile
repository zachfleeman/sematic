FROM ubuntu:20.04

WORKDIR /sema

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

# install link-grammar, so that the rust compiler can bind against it.
# RUN chmod u+r+x install_link_grammar.sh
# RUN ./install_link_grammar.sh
RUN /sema/link-parser-rust-bindings/link-grammar
RUN ./configure
RUN make
RUN make install

WORKDIR /sema
RUN cargo build --release

WORKDIR /sema/target/release

EXPOSE 8088
EXPOSE 443

CMD ["./sema-api"]