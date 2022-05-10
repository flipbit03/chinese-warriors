# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.231.2/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# (Rust stuff)
# 1. Add WebAssemly target
# 2. Add 2 cargo tools:
#   1. basic-http-server (to serve the wasm build)
#   2. cargo-watch (to automatically build stuff when source changes)
RUN rustup target add wasm32-unknown-unknown && \
    cargo install basic-http-server cargo-watch && \
    rm -rf /usr/local/cargo/registry
