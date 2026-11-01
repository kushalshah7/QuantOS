# Immutable multi-arch base pins Python and the included C/C++ build tools.
# Verified registry manifest: linux/amd64 and linux/arm64, 2026-11-01.
FROM python:3.12.15-bookworm@sha256:14a4933e6cef421424f7a66d89a3f4eafcbd0d90e0a822251f298e05132dc735

ENV CARGO_HOME=/home/quantos/.cargo
ENV RUSTUP_HOME=/home/quantos/.rustup
ENV PATH=/home/quantos/.cargo/bin:$PATH
ENV CARGO_BUILD_JOBS=2

RUN useradd --create-home --uid 1000 --shell /bin/bash quantos \
    && mkdir -p /workspaces/quantos \
    && chown quantos:quantos /workspaces/quantos

# .dockerignore allowlists only these public build inputs, never market data.
COPY rust-toolchain.toml package.json /tmp/quantos-toolchains/
COPY .devcontainer/install-toolchains.sh .devcontainer/downloads.sha256 /tmp/quantos-toolchains/
RUN bash /tmp/quantos-toolchains/install-toolchains.sh \
    && rm -rf /tmp/quantos-toolchains \
    && chown -R quantos:quantos /home/quantos

USER quantos
WORKDIR /workspaces/quantos
CMD ["sleep", "infinity"]
