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
        # Allow local connections to the X server (warning if not in a GUI session)
        if ! xhost +local:docker >/dev/null 2>&1; then
             echo "[Warning] Failed to connect to the X server with xhost. GUI apps inside the container might not work."
        fi
        get_service_name
        # Execute bash inside the corresponding service container
        docker compose exec -it "$SERVICE_NAME" bash "$@"
        ;;
    
    stop|down)
        docker compose down "$@"
        ;;

    restart)
        docker compose restart "$@"
        ;;

    remove|rm)
        docker compose rm -f -s -v "$@"
        ;;

    logs)
        get_service_name
        docker compose logs -f "$SERVICE_NAME" "$@"
        ;;
        
    *)
        echo "Usage: ./docker.sh {start|shell|stop|restart|remove|logs} [args...]"
        echo "  start   - Starts the docker container in exactly the same way as the old start.sh."
        echo "  shell   - Opens a bash shell within the running container."
        echo "  stop    - Stops and removes the container and network (down)."
        echo "  restart - Restarts the container."
        echo "  remove  - Stops and removes containers, leaving networks intact."
        echo "  logs    - Tails the container logs."
        exit 1
        ;;
esac