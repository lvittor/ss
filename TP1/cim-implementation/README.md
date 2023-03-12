# Build

`docker run --rm -it --user "$(id -u)":"$(id -g)" -e CARGO_HOME=/usr/src/myapp/cargo_home -v "$PWD":/usr/src/myapp -w /usr/src/myapp rustlang/rust:nightly-slim cargo build --all-targets`

# Run

## Implementation
`./target/debug/cim-implementation < ../data/input.txt > ../data/output.txt`

## Visualization
`./target/debug/visualization --input ../data/input.txt --output ../data/output.txt`
