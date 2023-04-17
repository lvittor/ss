
INPUT_FILE_PATH := TP3/data/input.txt
OUTPUT_FILE_PATH := TP3/data/output.txt
CAPTURE_DIR := 
RUN_ARGS :=
MAX_TIME :=
PRECISION := F64
FEATURES :=

ifeq "$(PRECISION)" "F64"
	FEATURES := "use_f64"
else
	FEATURES := "use_f32"
endif


build:
	make -C .. -f Makefile.rust build PACKAGE=tp3 "FEATURE_ARGS=--no-default-features --features=$(FEATURES)"

run-with-vis: build
	make -C .. -f Makefile.rust -s run-raw BIN=simulation \
		ARGS="--input $(INPUT_FILE_PATH) $(if $(MAX_TIME),--max-duration $(MAX_TIME))" | \
	make -C .. -f Makefile.rust run-raw BIN=visualization \
		ARGS="--input $(INPUT_FILE_PATH) --output /dev/stdin $(if $(CAPTURE_DIR),--capture-directory $(CAPTURE_DIR))"

run-raw:
	make -C .. -f Makefile.rust -s run-raw PACKAGE=tp3 ARGS="$(RUN_ARGS)"


.PHONY: build run-raw run-with-vis
