#!/bin/bash

# Wrapping the entire logic in a function ensures Bash parses and loads 
# everything into memory before execution, preventing EOF errors during self-updates.
main() {

# Load external configuration if present
source .eadkp/utils.sh
load_config

# Verify required dependencies before proceeding
REQUIRED_CMDS=("curl" "git" "realpath" "cmp" "just" "tr" "head" "sed" "grep" "cut")
MISSING_CMDS=()
for cmd in "${REQUIRED_CMDS[@]}"; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        MISSING_CMDS+=("$cmd")
    fi
done

if [ ${#MISSING_CMDS[@]} -ne 0 ]; then
    echo "[Dependencies] ERROR: The following required commands are missing: ${MISSING_CMDS[*]}"
    echo "Note: 'realpath' is not installed by default on macOS (can be installed via 'brew install coreutils')."
    echo "Note: 'just' must be installed manually (https://github.com/casey/just)."
    exit 1
fi

# Safety check to prevent running the script directly from within its folder
if [[ "$(basename "$PWD")" == "$DIR_NAME" ]]; then
    echo "[Safety] Please run the update script from the root directory of the project."
    exit 1
fi

# Get the list of files (name and hash) in the '$DIR_NAME' folder from the repo https://github.com/$REPO.git

# Declare a dictionary (associative array in bash)
declare -A file_hashes

# Request the GitHub API to get the folder content
# The -f option ignores HTTP errors (like 404) and returns an empty array
json_response=$(curl -s -f "https://api.github.com/repos/$REPO/contents/$DIR_NAME?ref=$BRANCH" || echo "FAILED")

if [[ "$json_response" == "FAILED" ]]; then
    echo "[Remote] ERROR: Could not fetch from GitHub API. Please check your internet connection or API rate limits."
    exit 1
fi
if [[ "$json_response" == "[]" ]]; then
    echo "[Remote] ERROR: The $DIR_NAME folder is empty or does not exist."
    exit 0
fi

# Extract file names, shas and types using grep/cut to completely avoid the 'jq' dependency
# Setting IFS enables supporting filenames with spaces natively up to bash 3.2
OLD_IFS="$IFS"
IFS=$'\n'
names=( $(echo "$json_response" | grep '"name":' | cut -d'"' -f4) )
shas=( $(echo "$json_response" | grep '"sha":' | cut -d'"' -f4) )
types=( $(echo "$json_response" | grep '"type":' | cut -d'"' -f4) )
IFS="$OLD_IFS"

for i in "${!names[@]}"; do
    if [[ "${types[$i]}" == "file" ]]; then
        file_hashes["${names[$i]}"]="${shas[$i]}"
    fi
done

# # Display the dictionary content to verify the results
# for file in "${!file_hashes[@]}"; do
#     echo "$file: ${file_hashes[$file]}"
# done


# Get the list of files (name and hash) in the local '$DIR_NAME' folder
declare -A local_file_hashes

# Function to check if a file is a text file natively
is_text_file() {
    # grep -I ignores binary files. Thus if '.' matches, it's text. Fast and file-dependency free.
    [[ -f "$1" ]] && head -c 1024 "$1" | grep -Iq .
}

if [[ -d "$DIR_NAME" ]]; then
    for filepath in "$DIR_NAME"/*; do
        if [[ -f "$filepath" ]]; then
            filename=$(basename "$filepath")
            # git hash-object calculates the SHA-1 exactly like GitHub
            # Ensure carriage returns (CRLF typically from WSL) don't falsify the blob hash by removing them
            # but only for text files to avoid corrupting binary hashes
            if is_text_file "$filepath"; then
                local_sha=$(tr -d '\r' < "$filepath" | git hash-object --stdin)
            else
                local_sha=$(git hash-object "$filepath")
            fi
            local_file_hashes["$filename"]="$local_sha"
        fi
    done
else
    echo "[Local] ERROR: The folder $DIR_NAME does not exist."
fi

# # Display the local dictionary content
# for file in "${!local_file_hashes[@]}"; do
#     echo "$file: ${local_file_hashes[$file]}"
# done


# Compare the two dictionaries
# If a file was modified, replace it;
# If it doesn't exist, download it; If it doesn't exist on the remote, delete it.
# Display the actions performed for each file

# Create the local folder if it doesn't exist so we can write into it
mkdir -p "$DIR_NAME"

# 1. Check remote files (to download or update)
for file in "${!file_hashes[@]}"; do
    if is_ignored_file "./$DIR_NAME/$file"; then
        # Skip the ignored file from being overwritten by remote templates
        continue
    fi

    remote_sha="${file_hashes[$file]}"
    local_sha="${local_file_hashes[$file]}"

    if [[ -z "$local_sha" ]]; then
        echo "[Updating] Downloading new file: $file"
        curl -s -L -o "$DIR_NAME/$file.tmp" "https://raw.githubusercontent.com/$REPO/$BRANCH/$DIR_NAME/$file"
        mv "$DIR_NAME/$file.tmp" "$DIR_NAME/$file"
    elif [[ "$remote_sha" != "$local_sha" ]]; then
        echo "[Updating] Updating modified file: $file"
        curl -s -L -o "$DIR_NAME/$file.tmp" "https://raw.githubusercontent.com/$REPO/$BRANCH/$DIR_NAME/$file"
        mv "$DIR_NAME/$file.tmp" "$DIR_NAME/$file"
    else
        echo "[Updating] The file $file is up to date."
    fi
done

# Ensure all scripts remain executable
chmod +x "$DIR_NAME"/*.sh 2>/dev/null || true

# 2. Check local files (to delete if not present on the remote)
for file in "${!local_file_hashes[@]}"; do
    if is_ignored_file "./$DIR_NAME/$file"; then
        # Ensure our local ignored files are never deleted even if absent from remote
        continue
    fi

    if [[ -z "${file_hashes[$file]}" ]]; then
        echo "[Updating] Deleting file that no longer exists on the remote: $file"
        rm -f "$DIR_NAME/$file"
    fi
done


# Update Cargo dependencies
echo "[Dependencies] Updating Cargo dependencies..."

if just --yes update; then
    echo "[Dependencies] Cargo dependencies updated successfully."
else
    echo "[Dependencies] ERROR: Failed to update Cargo dependencies."
    exit 1
fi

# Verify and auto-restore the root launchers
echo ""
echo "[Launchers] Verifying root launchers..."

# Fetch the root directory content 
root_json_response=$(curl -s -f "https://api.github.com/repos/$REPO/contents/?ref=$BRANCH" || echo "FAILED")

if [[ "$root_json_response" == "FAILED" ]]; then
    echo "[Launchers] ERROR: Could not fetch from GitHub API. Please check your internet connection or API rate limits."
    exit 1
fi

OLD_IFS="$IFS"
IFS=$'\n'
# Extract all names ending with .sh (using grep to avoid jq)
root_sh_scripts=( $(echo "$root_json_response" | grep '"name":' | cut -d'"' -f4 | grep '\.sh$' || true) )
IFS="$OLD_IFS"

for launcher in "${root_sh_scripts[@]}"; do
    if is_ignored_file "./$launcher"; then
        continue
    fi

    curl -s -L -o "$launcher.tmp" "https://raw.githubusercontent.com/$REPO/$BRANCH/$launcher"
    if [ -f "$launcher" ]; then
        if ! cmp -s "$launcher.tmp" "$launcher"; then
            echo "[Launchers] Updating modified root launcher: $launcher"
            mv "$launcher.tmp" "$launcher"
            chmod +x "$launcher"
        else
            rm -f "$launcher.tmp"
        fi
    else
        echo "[Launchers] Restoring missing root launcher: $launcher"
        mv "$launcher.tmp" "$launcher"
        chmod +x "$launcher"
    fi
done


# Final message

echo ""
echo "Updating process completed successfully!"
echo ""

}

# Execute the main payload that's now entirely loaded in memory
main "$@"
exit 0