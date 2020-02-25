FROM debian:buster AS builder

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y \
        build-essential \
        clang \
        curl \
        libcapstone-dev \
        libz3-dev \
        pkg-config \
        python3 \
        python3-pip \
        wget && \
    apt-get clean

RUN wget https://github.com/unicorn-engine/unicorn/archive/1.0.1.tar.gz && \
    tar xf 1.0.1.tar.gz && \
    cd unicorn-1.0.1 && \
    make && \
    make install

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh && \
    chmod +x /tmp/rustup.sh && \
    /tmp/rustup.sh -y --default-toolchain nightly

RUN pip3 install setuptools setuptools-rust

COPY . /falconre

RUN cd /falconre && \
    python3 setup.py install

FROM debian:buster

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y \
        libcapstone3 \
        libz3-4 \
        python3 \
        && \
    apt-get clean

COPY --from=builder /usr/local/lib/python3.7/dist-packages/falconre-0.1.0-py3.7-linux-x86_64.egg /usr/local/lib/python3.7/dist-packages/