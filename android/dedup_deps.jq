[.[] | {name, version: (.version | split(".") | map(tonumber))}]
| group_by(.name)
| map(max_by(.version))
| map({name: .name, version: (.version | join("."))})