#!/bin/bash

# Default configuration path
CONFIG_FILE=".eadkp/config.env"

# Function to load and validate the configuration file
load_config() {
    if [ ! -f "$CONFIG_FILE" ]; then
        echo "[Error] Configuration file $CONFIG_FILE not found."
        echo "Please refer to the documentation to configure your repository."
        exit 1
    fi

    source "$CONFIG_FILE"

    # Validate that all required variables are set
    if [ -z "$REPO" ] || [ -z "$BRANCH" ] || [ -z "$DIR_NAME" ]; then
        echo "[Error] Missing required variables (REPO, BRANCH, DIR_NAME) in $CONFIG_FILE."
        echo "Please check the file and refer to the documentation if needed."
        exit 1
    fi
}

# Function to logically resolve a path to an absolute path
get_absolute_path() {
    # Use realpath -m (or readlink -m as fallback) to resolve paths efficiently
    # The -m option ensures it works even if the file/directory doesn't exist yet
    realpath -m "$1" 2>/dev/null || readlink -m "$1"
}

# Function to check if a file should be ignored from updates
is_ignored_file() {
    local raw_target="$1"
    local target=$(get_absolute_path "$raw_target")
    
    # Sécurité : Si le chemin ne peut pas être résolu, on ne prend pas de risque
    [[ -z "$target" ]] && return 1
    
    # 1. Protection native config.env (utilisation de défaut si DIR_NAME vide)
    local config_path=$(get_absolute_path "${DIR_NAME:-.eadkp}/config.env")
    if [[ -n "$config_path" && "$target" == "$config_path" ]]; then
        return 0
    fi
    
    # 2. Vérification de la liste
    [[ -z "$IGNORE_FILES" ]] && return 1
    
    # Utilisation d'un séparateur local pour IFS pour ne pas polluer le reste du script
    (
        IFS=','
        for entry in $IGNORE_FILES; do
            # Trim manuel sans sous-processus pour la performance
            entry="${entry#"${entry%%[![:space:]]*}"}"
            entry="${entry%"${entry##*[![:space:]]}"}"
            
            [[ -z "$entry" ]] && continue
            
            local resolved_entry=$(get_absolute_path "$entry")
            if [[ -n "$resolved_entry" && "$target" == "$resolved_entry" ]]; then
                exit 0 # Trouvé !
            fi
        done
        exit 1 # Non trouvé
    )
    return $?
}

# Function to extract the project name from Cargo.toml and export it
export_project_name() {
    # Match both double and single quotes using sed
    export PROJECT_NAME=$(grep -m 1 '^name *=' Cargo.toml | sed -E "s/^name *= *['\"]([^'\"]+)['\"].*/\1/")

    if [ -z "$PROJECT_NAME" ]; then
        echo "[Error] Could not find the project name in Cargo.toml."
        exit 1
    fi
}
