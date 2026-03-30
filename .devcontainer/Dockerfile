FROM rust

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        fd-find \
        fzf \
        git \
        gnupg2 \
        jq \
        just \
        lldb \
        less \
        man-db \
        musl-tools \
        netcat-openbsd \
        neovim \
        openssl \
        pkg-config \
        procps \
        python-is-python3 \
        python3 \
        ripgrep \
        sudo \
        tmux \
        tree \
        unzip \
        vim \
        zsh \
    && ln -sf /usr/bin/fdfind /usr/local/bin/fd \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt \
    && rustup target add x86_64-unknown-linux-musl

RUN cargo install --locked \
        cargo-audit \
        cargo-deb \
        cargo-deny \
        cargo-edit \
        cargo-expand \
        cargo-generate-rpm \
        cargo-llvm-cov

WORKDIR /workspaces
