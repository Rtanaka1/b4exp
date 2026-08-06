#!/bin/bash

# Define the target subdirectory
TARGET_DIR="/home/crates/sources"

# Create the subdirectory if it doesn't exist
mkdir -p "$TARGET_DIR"

# Loop through all .tar.gz files in the current directory
for file in /home/crates/*.tar.gz; do
    if [[ -f "$file" ]]; then
        echo "Unarchiving $file to $TARGET_DIR/"
        # Extract the contents to the target subdirectory
        tar -xzf "$file" -C "$TARGET_DIR"
    else
        echo "No .tar.gz files found."
    fi
done

echo "All files have been unarchived to $TARGET_DIR/"
