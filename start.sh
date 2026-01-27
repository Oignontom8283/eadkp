#!/bin/bash

# Check docker is installed
if ! command -v docker &> /dev/null
then
    echo "Docker could not be found. Please install Docker to proceed."
    exit 1
fi

# Allow local connections to the X server
xhost +local:docker

# Start the Docker container with GUI support and pass all arguments
docker compose up -d "$@"
