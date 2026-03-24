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

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Install Rust toolchain
RUN rustup toolchain install nightly
RUN rustup show

# Add ARM target
RUN rustup target add thumbv7em-none-eabihf
RUN rustup target add thumbv7em-none-eabihf --toolchain nightly

# Install cargo tools
RUN cargo install just
RUN cargo install cargo-edit

# Add user to dialout group for USB access
RUN usermod -aG dialout root

WORKDIR /workspace
