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
CRATE_ARCHIVE_DIR="${CRATE_BUILD_DIR}/archives"
mkdir -p $CRATE_ARCHIVE_DIR
AVAILABLE_CRATES="${SCRIPT_DIR}/../../crates"

## Phase 1: Generate Android.bp for crate
#
## TODO: Add ability to parse some command line args to trigger only certain parts
#cargo_embargo autoconfig cargo_embargo.json
#
## Phase 2: Generate listing of deps for crate + their download URLs
#
#cargo tree --prefix=none | "${SCRIPT_DIR}/cargo_to_json_deps.sh" > "${ANDROID_BUILD_DIR}/deps.json"
#jq -f "${SCRIPT_DIR}/dedup_deps.jq" "${ANDROID_BUILD_DIR}/deps.json" > "${ANDROID_BUILD_DIR}/deduped_deps.json"
#jq -f "${SCRIPT_DIR}/add_dl_urls.jq" "${ANDROID_BUILD_DIR}/deduped_deps.json" > "${ANDROID_BUILD_DIR}/deduped_deps_urls.json"
#
## Phase 3: Download all the crates
#
#jq -c '.[]' "${ANDROID_BUILD_DIR}/deduped_deps_urls.json" | while read -r i; do
#    url=$(echo "$i" | jq -r '.url')
#    file_name=$(basename "$url")
#    save_path="$CRATE_BUILD_DIR/$file_name"
#
#    echo "Downloading $url..."
#    curl -o "$save_path" "$url"
#done
#
## Phase 4: Unzip crates
#
#find $CRATE_ARCHIVE_DIR -name '*.crate' -exec tar -xzf {} -C $CRATE_BUILD_DIR \;

# Phase 5: Run `cargo_embargo autoconfig cargo_embargo.json` on each folder

#original_dir=$(pwd)
#
## Iterate over each folder in CRATE_BUILD_DIR
#find "$CRATE_BUILD_DIR" -mindepth 1 -maxdepth 1 -type d | while read -r dir; do
#    echo "Entering directory $dir"
#    cd "$dir" || exit
#
#    # Run cargo_embargo command
#    echo "Running 'cargo_embargo autoconfig cargo_embargo.json' in $dir"
#    cargo_embargo autoconfig cargo_embargo.json
#
#    # Return to the original directory
#    cd "$original_dir" || exit
#done
#
#echo "Completed processing all directories."

## Phase 6: Catalog successes and failures to understand which crates
# we will be able to make an Android.bp for: NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS
# and those we will not be able to: NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS

# Initialize arrays to hold directories based on their status
NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS=()
NOT_AVAILABLE_CARGO_EMBARGO_FAIL=()

# Loop over each folder in CRATE_BUILD_DIR
for crate_dir in "$CRATE_BUILD_DIR"/*; do
    if [ -d "$crate_dir" ]; then
        crate_name_version=$(basename "$crate_dir")
        crate_name=$(echo "$crate_name_version" | sed -E 's/-[0-9]+\.[0-9]+\.[0-9]+$//')

        # Check if the crate is not available in AVAILABLE_CRATES
        if [ ! -d "$AVAILABLE_CRATES/$crate_name" ]; then
            # Further check for the presence of cargo_embargo.json
            if [ -f "$crate_dir/cargo_embargo.json" ]; then
                NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS+=("$crate_name_version")
            else
                NOT_AVAILABLE_CARGO_EMBARGO_FAIL+=("$crate_name_version")
            fi
        fi
    fi
done

# Output the results
echo "Crates NOT_AVAILABLE but have cargo_embargo.json:"
for crate in "${NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS[@]}"; do
    echo "$crate"
done

echo "Crates NOT_AVAILABLE and missing cargo_embargo.json:"
for crate in "${NOT_AVAILABLE_CARGO_EMBARGO_FAIL[@]}"; do
    echo "$crate"
done

# Phase 7: For any NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS, we want to catalog the build deps and then search for
#  if any of those have their deps completely covered by what's available in AVAILABLE_CRATES and put those into
#  a list called MUST_BUILD_CRATES_AVAILABLE and for those that are not completely covered, put them in a list
#  called MUST_BUILD_CRATES_NOT_AVAILABLE with which deps are NOT_AVAILABLE

# Assuming paths and previous phases are correctly set up
# Initialize arrays for the new phase
MUST_BUILD_CRATES_AVAILABLE=()
MUST_BUILD_CRATES_NOT_AVAILABLE=()

# Function to check if dependencies are available
check_deps_available() {
    local crate_name="$1"
    local deps_file="$2" # Path to the JSON file with dependencies
    local all_deps_available=true
    local unavailable_deps=()

    echo "check_deps_available for crate: ${crate_name}"

    # Read dependencies from the JSON file
    local deps=$(jq -r '.[] | .name' "$deps_file")

    for dep in $deps; do
        echo "checking dep: ${dep}"
        if [ ! -d "$AVAILABLE_CRATES/$dep" ]; then
            echo "dep unavailable: ${dep}"
            all_deps_available=false
            unavailable_deps+=("$dep")
        fi
    done

    if $all_deps_available; then
        echo "all deps available for crate: ${crate_name}"
        MUST_BUILD_CRATES_AVAILABLE+=("$crate_name")
    else
        echo "all deps not available for crate: ${crate_name}"
        echo "$crate_name: ${unavailable_deps[*]}" >> "${CRATE_BUILD_DIR}/MUST_BUILD_CRATES_NOT_AVAILABLE.txt"
    fi
}

# Process each NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS crate
for crate in "${NOT_AVAILABLE_CARGO_EMBARGO_SUCCESS[@]}"; do
    # Assume a hypothetical command to generate a JSON file of dependencies for the crate
    DEPS_JSON="${CRATE_BUILD_DIR}/${crate}_deps.json"
    DEDUPED_DEPS_JSON="${CRATE_BUILD_DIR}/${crate}_deduped_deps.json"
    cargo tree --manifest-path "${CRATE_BUILD_DIR}/${crate}/Cargo.toml" --prefix=none | "${SCRIPT_DIR}/cargo_to_json_deps.sh" > "$DEPS_JSON"
    jq -f "${SCRIPT_DIR}/dedup_deps.jq" "$DEPS_JSON" > "$DEDUPED_DEPS_JSON"

    # Now check if all dependencies for this crate are available
    check_deps_available "$crate" "$DEDUPED_DEPS_JSON"
done

# Output the results
echo "Crates that must be built and have all dependencies available:"
for crate in "${MUST_BUILD_CRATES_AVAILABLE[@]}"; do
    echo "$crate"
done

echo "Crates that must be built but do not have all dependencies available (see MUST_BUILD_CRATES_NOT_AVAILABLE.txt for details)."

# Notes on deps completed and moved to external/rust/crates:
# => autocfg
# => cc
# => doc-comment
#  ~ had to comment out beginning of lib.rs configuring to be no_std
# => home