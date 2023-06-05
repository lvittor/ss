
INPUT_FILE_PATH := TP5/data/input.txt
OUTPUT_FILE_PATH := TP5/data/output.txt
CAPTURE_DIR := 
RUN_ARGS :=
MAX_TIME :=
STEPS_PER_SECOND := 1000
EVERY := 10

CAPTURE_DIR_ARG := $(if $(CAPTURE_DIR),--capture-directory $(CAPTURE_DIR))


build:
	make -C .. -f Makefile.rust build PACKAGE=tp5

run-with-vis: build
	make -C .. -f Makefile.rust -s run-raw BIN=simulation \
		ARGS="--input $(INPUT_FILE_PATH) --output-exit-times=/dev/null --output-particles=/dev/stdout --steps-per-second=$(STEPS_PER_SECOND) --output-every $(EVERY) --output-last" | \
	make -C .. -f Makefile.rust run-raw BIN=visualization \
		ARGS="--input $(INPUT_FILE_PATH) --output /dev/stdin $(CAPTURE_DIR_ARG)"

run-raw:
	make -C .. -f Makefile.rust -s run-raw PACKAGE=tp5 ARGS="$(RUN_ARGS)"

.PHONY: build run-raw run-with-vis run-with-vis2