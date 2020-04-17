FROM debian:buster AS builder

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y \
        build-essential \
        clang \
        curl \
        libz3-dev \
        pkg-config \
        python3 \
        python3-pip \
        wget && \
    apt-get clean

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN wget https://github.com/aquynh/capstone/archive/4.0.1.tar.gz && \
    tar xf 4.0.1.tar.gz && \ 
    cd capstone-4.0.1 && \
    make -j 4 && make install

RUN pip3 install setuptools setuptools-rust

WORKDIR /falconre
COPY ./src /falconre/src
COPY ./Cargo.toml /falconre/Cargo.toml
COPY ./Cargo.lock /falconre/Cargo.lock
COPY ./falconre/ /falconre/falconre
COPY ./setup.py /falconre/setup.py

RUN python3 setup.py install

FROM debian:buster

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y \
        libz3-4 \
        python3 \
        python3-pip && \
    apt-get clean

COPY --from=builder /usr/lib/libcapstone.so.4 /usr/lib/
COPY --from=builder /usr/local/lib/python3.7/dist-packages/falconre-0.1.0-py3.7-linux-x86_64.egg /usr/local/lib/python3.7/dist-packages/

COPY requirements.txt .

RUN pip3 install -r requirements.txt