FROM debian:stretch

ARG branch
ENV branch=$branch
WORKDIR /usr/src/
# Install dependencies
RUN apt update && apt install -y \
  git \
  gcc \
  pkg-config \
  bash \
  curl

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
  chmod +x /tmp/rust.sh && \
  /tmp/rust.sh -y
# Clone repository
RUN git clone --branch $branch https://github.com/veeso/tuifeed.git
# Set workdir to tuifeed
WORKDIR /usr/src/tuifeed/
# Install cargo deb
RUN . $HOME/.cargo/env && cargo install cargo-deb
# Build for x86_64
RUN . $HOME/.cargo/env && cargo build --release
# Build pkgs
RUN . $HOME/.cargo/env && cargo deb

CMD ["bash"]
