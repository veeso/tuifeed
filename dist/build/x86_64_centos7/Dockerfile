FROM centos:centos7 as builder

WORKDIR /usr/src/
# Install dependencies
RUN yum -y install \
    git \
    gcc \
    pkgconfig \
    curl \
    bash
# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
    chmod +x /tmp/rust.sh && \
    /tmp/rust.sh -y
# Clone repository
RUN git clone https://github.com/veeso/tuifeed.git
# Set workdir to tuifeed
WORKDIR /usr/src/tuifeed/
# Install cargo arxch
RUN source $HOME/.cargo/env && cargo install cargo-rpm cargo-strip
# Build for x86_64
RUN source $HOME/.cargo/env && cargo build --release && cargo strip
# Build pkgs
RUN source $HOME/.cargo/env && yum -y install rpm-build && cargo rpm init && cargo rpm build
CMD ["sh"]
