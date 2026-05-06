#!/bin/bash

# Target directory containing the internal scripts
TARGET_DIR=".eadkp"
TARGET_SCRIPT="$TARGET_DIR/$(basename "$0")"

# Move to the script's directory in case it is called from another location
cd "$(dirname "$0")"

# Pass execution to the internal script and forward any given arguments
exec "./$TARGET_SCRIPT" "$@"