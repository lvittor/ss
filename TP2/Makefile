
INPUT_FILE_PATH := TP2/data/input.txt
OUTPUT_FILE_PATH := TP2/data/output.txt
CAPTURE_DIR := 
ARGS :=

build:
	make -C .. -f Makefile.rust build PACKAGE=tp2

run-with-vis: build
	make -C .. -f Makefile.rust -s run-raw BIN=simulation ARGS="--input $(INPUT_FILE_PATH)" | \
	make -C .. -f Makefile.rust run-raw BIN=visualization ARGS="--input $(INPUT_FILE_PATH) --output /dev/stdin $(if $(CAPTURE_DIR),--capture-directory $(CAPTURE_DIR))"


.PHONY: build run-with-vis
