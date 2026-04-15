FROM debian:bookworm

# Build tools & deps
RUN apt-get update && apt-get install -y \
    build-essential cmake git curl wget unzip usbutils \
    gcc-arm-none-eabi binutils-arm-none-eabi gdb-multiarch pkg-config libpng-dev libjpeg-dev libfreetype6-dev \
    python3 python3-pip \
    libusb-1.0-0 libusb-1.0-0-dev \
    libudev-dev python-is-python3 \
    nodejs npm \
    libx11-dev libxext-dev libxrender-dev libxrandr-dev libxinerama-dev \
    libgl1-mesa-dev libglu1-mesa-dev \
    libpng-dev libjpeg-dev python3-lz4 \
    imagemagick lz4 jq \
    micro nano \
    && rm -rf /var/lib/apt/lists/*

RUN npm install -g --unsafe-perm usb nwlink

# Create a user 'dev' and add it to the 'dialout' group for USB access
RUN useradd -m -s /bin/bash dev && \
    usermod -aG dialout dev

# Install Rust (in /opt/rust)
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
ENV PATH="/opt/cargo/bin:${PATH}"

# Set permissions for Rust
RUN chmod -R 777 /opt/cargo /opt/rustup

# RUN rustup toolchain install nightly && \
#     rustup target add thumbv7em-none-eabihf && \
#     rustup target add thumbv7em-none-eabihf --toolchain nightly

RUN rustup target add thumbv7em-none-eabihf

# Install cargo tools
RUN cargo install just cargo-edit

WORKDIR /workspace