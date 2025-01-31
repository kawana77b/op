#!/bin/bash

set -e

APP_NAME=op
test -n "$APP_VERSION" || APP_VERSION="v0.0.0"

rm -rf dist
mkdir -p dist

TARGET_DIR="target"
ROOT_DIR="$(pwd)"
DIST_DIR="$(pwd)/dist"

# Collect binaries for each target folder and move them to the dist folder.
# Also copy documents to that folder.
cd $TARGET_DIR
for dir in */; do
    dir=${dir%/}
    [[ "$dir" == "CHACHEDIR.TAG" ]] && continue
    
    binary_path="$dir/release/${APP_NAME}"
    [[ -f "$binary_path.exe" ]] && binary_path+=".exe"
    [[ -f "$binary_path" ]] || { echo "Binary not found in $dir"; continue; }
    
    target_dir="${DIST_DIR}/$dir"
    mkdir -p "$target_dir"
    
    cp "$binary_path" "$target_dir/"
    cp "${ROOT_DIR}/README.md" "${ROOT_DIR}/CREDITS" "$target_dir/"
done

# Archive each target folder
cd $DIST_DIR
for dir in */; do
    dir=${dir%/}
    archive_name="${APP_NAME}_${dir}_${APP_VERSION}"
    if [[ "$dir" == *windows* ]]; then
        zip -r "${archive_name}.zip" -j "$dir"
    else
        tar -czf "${archive_name}.tar.gz" -C "$dir" .
    fi
done
