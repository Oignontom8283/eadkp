FROM debian:bookworm

ARG DEBIAN_FRONTEND=noninteractive

# Build tools & deps
RUN apt-get update && apt-get install -y \
    build-essential cmake git curl wget unzip usbutils sudo \
    gcc-arm-none-eabi binutils-arm-none-eabi gdb-multiarch pkg-config libpng-dev libjpeg-dev libfreetype6-dev \
    python3 python3-pip \
    libusb-1.0-0 libusb-1.0-0-dev \
    libudev-dev python-is-python3 \
    nodejs npm \
    libx11-dev libxext-dev libxrender-dev libxrandr-dev libxinerama-dev \
    libgl1-mesa-dev libglu1-mesa-dev \
    python3-lz4 imagemagick lz4 jq \
    micro nano \
    && rm -rf /var/lib/apt/lists/*

RUN npm install -g --unsafe-perm usb nwlink

#  Create a non-root user
ARG USER_ID=1000
ARG GROUP_ID=1000
RUN groupadd -g ${GROUP_ID} dev_group || true && \
    useradd -l -u ${USER_ID} -g ${GROUP_ID} -m dev || \
    (usermod -u ${USER_ID} dev && groupmod -g ${GROUP_ID} $(id -gn dev))

# Allow the dev user to use sudo without a password
RUN echo "dev ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/dev && \
    chmod 0440 /etc/sudoers.d/dev

# Switch to the dev user to correctly configure Cargo/Rust toolchains right into its home directory
USER dev
ENV USER=dev
ENV HOME=/home/dev
ENV PATH="/home/dev/.cargo/bin:${PATH}"

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

# Add required target
RUN rustup target add thumbv7em-none-eabihf

# Install cargo tools
RUN cargo install just cargo-edit

WORKDIR /workspace