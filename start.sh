#!/bin/bash

# Check docker is installed
if ! command -v docker &> /dev/null
then
    echo "Docker could not be found. Please install Docker to proceed."
    exit 1
fi

# Allow local connections to the X server
xhost +local:docker

# Set permissions for Wayland socket
if [ -n "$WAYLAND_DISPLAY" ] && [ -n "$XDG_RUNTIME_DIR" ]; then
    WAYLAND_PATH="${XDG_RUNTIME_DIR}/${WAYLAND_DISPLAY}"
    if [ -e "$WAYLAND_PATH" ]; then
        echo "Configuring Wayland socket permissions..."
        sudo chmod 666 "$WAYLAND_PATH"
    fi
else
    echo "Wayland not detected, running in X11 mode."
fi

# Start the Docker container with GUI support and pass all arguments
docker compose up -d "$@"
