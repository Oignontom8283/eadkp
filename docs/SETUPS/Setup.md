## Dockerized Development Environment Setup Guide

This guide helps you set up the development environment for Eadkp.

## Prerequisites
- **Docker** and Docker Compose installed on your machine (If you are on Windows, install Docker Desktop on Windows and not in WSL).
- **Git** installed to clone the repository (If you are on Windows, install Git in WSL).
- A code editor supporting Rust and Docker integration (e.g., VSCode with the Dev Containers extension).

## Installation

### Clone the repository

```bash
git clone https://github.com/Oignontom8283/eadkp.git
cd eadkp
```

> [!IMPORTANT]
> On Windows, use WSL and run the commands in WSL.

## Launch

A `start.sh` file is provided to automate the start of the Docker environment.

```bash
chmod +x start.sh
```
```bash
./start.sh
```

> [!NOTE]
> Building the Docker image may take some time on the first launch because it includes many dependencies.

## Access the container

Once the container is launched, to start developing you need to enter the container.

### VSCode

On your main OS, open VSCode.
Download the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension.

Connect to your Docker container via VSCode using the extension.

![Connect to container](.//assets/Tuto_dev-container_1.png)

You can now work inside your Docker container.

### Terminal
In your terminal, use the `shell.sh` script to access the Docker container.

```bash
chmod +x shell.sh
./shell.sh
```

## Next steps

- [Compile the test example (export)](./BuildExample.md)
- [Use the test example in a simulator](./Simulator.md)