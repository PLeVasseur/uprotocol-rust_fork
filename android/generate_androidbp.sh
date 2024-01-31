#!/bin/bash

# Requirements (normal for an Android project)
# 1. Have repo cloned the Android Open Source Project
# 2. Built cargo_embargo (refer to its docs)
# 3. `source build/envsetup.sh` from AOSP root
# 4. jq installed

LIB_MIDFIX=_up_rust_
SCRIPT_DIR=$(dirname "$0")
ANDROID_BUILD_DIR="${SCRIPT_DIR}/build"
mkdir -p $ANDROID_BUILD_DIR
CRATE_BUILD_DIR="${ANDROID_BUILD_DIR}/crate_deps"
mkdir -p $CRATE_BUILD_DIR

# Phase 1: Generate Android.bp for crate

# TODO: Add ability to parse some command line args to trigger only certain parts
cargo_embargo autoconfig cargo_embargo.json

# Phase 2: Generate listing of deps for crate + their download URLs

cargo tree --prefix=none | "${SCRIPT_DIR}/cargo_to_json_deps.sh" > "${ANDROID_BUILD_DIR}/deps.json"
jq -f "${SCRIPT_DIR}/dedup_deps.jq" "${ANDROID_BUILD_DIR}/deps.json" > "${ANDROID_BUILD_DIR}/deduped_deps.json"
jq -f "${SCRIPT_DIR}/add_dl_urls.jq" "${ANDROID_BUILD_DIR}/deduped_deps.json" > "${ANDROID_BUILD_DIR}/deduped_deps_urls.json"

# Phase 3: Download all the crates

jq -c '.[]' "${ANDROID_BUILD_DIR}/deduped_deps_urls.json" | while read -r i; do
    url=$(echo "$i" | jq -r '.url')
    file_name=$(basename "$url")
    save_path="$CRATE_BUILD_DIR/$file_name"

    echo "Downloading $url..."
    curl -o "$save_path" "$url"
done

# Phase 4: Unzip crates

