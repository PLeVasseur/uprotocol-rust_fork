#!/bin/bash

# Requirements (normal for an Android project)
# 1. Have repo cloned the Android Open Source Project
# 2. Built cargo_embargo (refer to its docs)
# 3. `source build/envsetup.sh` from AOSP root
# 4. jq installed

SCRIPT_DIR=$(dirname "$0")
ANDROID_BUILD_DIR="${SCRIPT_DIR}/build"
mkdir -p $ANDROID_BUILD_DIR

# TODO: Add ability to parse some command line args to trigger only certain parts
# cargo_embargo autoconfig cargo_embargo.json
jq -f "${SCRIPT_DIR}/extract_deps.jq" "${SCRIPT_DIR}/../cargo.metadata" > "${ANDROID_BUILD_DIR}/deps.json"