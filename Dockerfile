FROM rust:1.90.0-trixie AS builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y --no-install-recommends \
    build-essential \
    bash \
    libclang-dev

RUN rustup component add clippy-preview \
    && rustup component add rustfmt

# Create user
ARG UID
ARG GID
RUN groupadd -g ${GID} satoshi && \
	useradd -m -r -u ${UID} -g ${GID} -G users,lp,disk,adm,dialout -c "Satoshi Nakamoto" -s /bin/bash satoshi
USER satoshi
WORKDIR /app