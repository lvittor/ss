## Build and run:

`cargo run < ../data/example_input.txt`

## Build and run in docker with:

`docker run --rm -i --user "$(id -u)":"$(id -g)" -e CARGO_HOME=/usr/src/myapp/cargo_home -v "$PWD":/usr/src/myapp -w /usr/src/myapp rustlang/rust:nightly-slim cargo run < ../data/example_input.txt`
