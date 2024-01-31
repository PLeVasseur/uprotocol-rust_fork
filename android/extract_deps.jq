def clean_version: ltrimstr("^") | if test("^[0-9]+(\\.[0-9]+)*$") then split(".") | map(tonumber) else error("Invalid version format: \(.)") end;

[.packages[] | {name: .name, version: (try (.version | clean_version) catch .)}] +
[.packages[].dependencies[] | {name: .name, version: (try (.req | clean_version) catch .)}] |
group_by(.name) |
map((max_by(.version | arrays) // error("Comparison error in \(.name)")) | {name: .name, version: (.version | arrays | join("."))})