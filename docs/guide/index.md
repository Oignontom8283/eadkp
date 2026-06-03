
# Getting Started 

Create a new project powered by Eadkp.

## Prerequisites
- Docker (On Windows, install Docker Desktop on Windows and not in WSL)
- Git (On Windows, install Git in WSL)

## Creation

To simplify project creation, we will use an automation script that builds a base project for us.

```bash
bash <(curl -s https://raw.githubusercontent.com/Oignontom8283/eadkp_template/main/bootstrap.sh) --name "my_app"
```
```bash
cd my_app
```

> [!IMPORTANT]
> On Windows, use WSL and run the commands in WSL.

## Launch

The base project uses Docker to simplify installation of the environment required to run the application.

```bash
chmod +x ./docker.sh
```
```bash
./docker.sh start
```

> [!NOTE]
> Docker includes many dependencies, so the first build can take a long time. Please be patient.

## Accessing the docker container

Once the Docker container is running, to start developing you need to enter the container.

### VSCode

Recommended to install the [Remote Development](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack)
extension pack in Visual Studio Code or [this](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension.

```bash
./docker.sh open code
```

This will open VSCode directly in the Docker container via Remote Development.

You can now work inside your Docker container as if you were working on your local machine.

### Terminal

In your terminal, use the script to access the Docker container.

```bash
./docker.sh shell
```