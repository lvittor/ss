.PHONY: help
.DEFAULT_GOAL := help

CARGO_HOME := /usr/src/myapp/cargo_home
WORKDIR := /usr/src/myapp
DOCKER_IMG := rustlang/rust:nightly-slim

INPUT_FILE_PATH := ../data/input.txt
OUTPUT_FILE_PATH := ../data/output.txt
TARGET_DIR := ./target/debug

ARGS := -o /dev/stdout -c


help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

build: ## Build all the binaries
	docker run --rm -it --user $(id -u):$(id -g)"" -e CARGO_HOME=$(CARGO_HOME) -v "$(PWD)":$(WORKDIR) -w $(WORKDIR) $(DOCKER_IMG) cargo build --all-targets

run-impl: ## Run the implementation binary. Usage: make run-impl [INPUT_FILE_PATH="file/path" | ARGS="--help"]
	$(TARGET_DIR)/cim-implementation --input $(INPUT_FILE_PATH) $(ARGS)

run-viz: ## Run the visualization binary. Usage: make run-viz [INPUT_FILE_PATH="file/path" | OUTPUT_FILE_PATH="file/path"]
	$(TARGET_DIR)/visualization --input $(INPUT_FILE_PATH) --output $(OUTPUT_FILE_PATH)