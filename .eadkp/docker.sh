#!/bin/bash

# Source utilities and export project name
source .eadkp/utils.sh
export_project_name

# Retrieve the command and shift the arguments
COMMAND=$1
shift

# Function to get the main service name
get_service_name() {
    SERVICE_NAME=$(docker compose config --services 2>/dev/null | head -n 1)
    if [ -z "$SERVICE_NAME" ]; then
        echo "[Error] Could not find any service in docker-compose.yml."
        exit 1
    fi
}

# Function to get the container ID of the main service
get_container_id() {
    get_service_name
    CONTAINER_ID=$(docker compose ps -q "$SERVICE_NAME" 2>/dev/null | head -n 1)
    if [ -z "$CONTAINER_ID" ]; then
        echo "[Error] Could not find a running container for service '$SERVICE_NAME'. Is it started?"
        exit 1
    fi
}

# Function to check that the container is running before executing a command
require_running() {
    get_service_name
    STATUS=$(docker compose ps --status running -q "$SERVICE_NAME" 2>/dev/null | head -n 1)
    if [ -z "$STATUS" ]; then
        echo "[Error] The container is not running. Please start it first with:"
        echo "  ./docker.sh start"
        exit 1
    fi
}

case "$COMMAND" in
    start|up)
        # Check docker is installed
        if ! command -v docker &> /dev/null; then
            echo "[Error] Docker could not be found. Please install Docker to proceed."
            exit 1
        fi
        # Allow local connections to the X server (warning if not in a GUI session)
        if ! xhost +local:docker >/dev/null 2>&1; then
             echo "[Warning] Failed to connect to the X server with xhost. GUI apps inside the container might not work."
        fi
        # Export user IDs for proper file permissions inside the container
        export HOST_UID=$(id -u)
        export HOST_GID=$(id -g)
        # Ensure XDG_RUNTIME_DIR is set for Wayland fallback
        export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/run/user/$HOST_UID}
        # Ensure the Wakaatime config file exists
        if [ ! -f "$HOME/.wakatime.cfg" ]; then
            touch "$HOME/.wakatime.cfg"
        fi
        # Start the Docker container with GUI support and pass all arguments
        docker compose up -d "$@"
        ;;
    
    shell|sh)
        require_running
        # Allow local connections to the X server (warning if not in a GUI session)
        if ! xhost +local:docker >/dev/null 2>&1; then
             echo "[Warning] Failed to connect to the X server with xhost. GUI apps inside the container might not work."
        fi
        get_service_name
        # Execute bash inside the corresponding service container
        docker compose exec -it "$SERVICE_NAME" bash "$@"
        ;;
    
    stop)
        require_running
        docker compose stop "$@"
        ;;
        
    down)
        require_running
        docker compose down "$@"
        ;;

    restart)
        require_running
        docker compose restart "$@"
        ;;

    remove|rm)
        require_running
        docker compose rm -f -s -v "$@"
        ;;

    logs)
        require_running
        get_service_name
        docker compose logs -f "$SERVICE_NAME" "$@"
        ;;

    open)
        require_running
        SUBCOMMAND=$1
        shift
        case "$SUBCOMMAND" in
            code)
                # Check VS Code is installed
                if ! command -v code &> /dev/null; then
                    echo "[Error] VS Code ('code') could not be found. Please install it to proceed."
                    exit 1
                fi
                get_container_id
                CONTAINER_HEX=$(printf '%s' "$CONTAINER_ID" | xxd -p | tr -d '\n')
                WORKSPACE=${1:-/workspace/$PROJECT_NAME}
                code --folder-uri "vscode-remote://attached-container+${CONTAINER_HEX}${WORKSPACE}"
                ;;
            *)
                echo "Usage: ./docker.sh open {code} [workspace_path]"
                echo "  code  - Opens VS Code attached to the running container."
                echo "          workspace_path defaults to /workspace/${PROJECT_NAME}"
                exit 1
                ;;
        esac
        ;;
        
    *)
        echo "Usage: ./docker.sh {start|shell|stop|restart|remove|logs|open} [args...]"
        echo "  start   - Starts the docker container in exactly the same way as the old start.sh."
        echo "  shell   - Opens a bash shell within the running container."
        echo "  stop    - Stops and removes the container and network (down)."
        echo "  restart - Restarts the container."
        echo "  remove  - Stops and removes containers, leaving networks intact."
        echo "  logs    - Tails the container logs."
        echo "  open    - Opens an IDE attached to the running container."
        echo "            Subcommands: code [workspace_path]"
        exit 1
        ;;
esac