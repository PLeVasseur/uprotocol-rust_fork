# Assuming input is an array of two elements: [prev_deps, curr_deps]
.[0] as $prev | .[1] as $curr |
# Map dependencies by name for easy lookup
($prev | map({(.name): .version}) | add) as $prevDeps |
($curr | map({(.name): .version}) | add) as $currDeps |
# Combine all keys (dependency names) and filter for unique ones
($prevDeps + $currDeps | keys_unsorted | unique) as $allDeps |
# Iterate over each dependency to check for changes or new additions
[ $allDeps[] as $dep |
  # Check if the dependency exists in both previous and current sets
  if ($prevDeps[$dep] and $currDeps[$dep]) then
    # If versions differ, note the change
    if ($prevDeps[$dep] != $currDeps[$dep]) then
      { name: $dep, prev_version: $prevDeps[$dep], new_version: $currDeps[$dep], fail_reason: "changed_version" }
    else
      empty
    end
  elif ($currDeps[$dep]) then
    # Dependency is new in current set
    { name: $dep, version: $currDeps[$dep], fail_reason: "new_dep" }
  else
    empty
  end
] | unique | if length == 0 then "[]" else . end