#!/bin/bash

# Function to convert cargo output to JSON
cargo_to_json() {
    echo '['
    local first=true
    local skip_first_line=true
    while read -r line; do
        if [ "$skip_first_line" = true ]; then
            skip_first_line=false
            continue
        fi
        if [[ $line =~ ([^[:space:]]+)[[:space:]]+v([0-9.]+) ]]; then
            if [ "$first" = true ]; then
                first=false
            else
                echo ','
            fi
            echo -n '    {"name": "'${BASH_REMATCH[1]}'", "version": "'${BASH_REMATCH[2]}'"}'
        fi
    done
    echo
    echo ']'
}

# Reading from stdin
cargo_tree_output=$(cat)
echo "$cargo_tree_output" | cargo_to_json