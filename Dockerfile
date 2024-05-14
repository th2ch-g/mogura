FROM rust:latest

RUN apt-get update && \
        apt-get install -y \
        x11-apps \
        libatk1.0-dev \
        libgdk-pixbuf2.0-dev \
        libgtk-3-dev \
        libcairo2-dev \
        libpango1.0-dev \
        libxkbcommon-x11-0

COPY . /mogura

RUN cd /mogura && cargo build -r

ENV PATH="/mogura/target/release:${PATH}"
