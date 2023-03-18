build:
	make -C .. -f Makefile.rust build PACKAGE=tp1

INPUT_FILE_PATH := TP1/data/input.txt
OUTPUT_FILE_PATH := TP1/data/output.txt
ARGS :=

build:
	make -C .. -f Makefile.rust -s build PACKAGE=tp1

run-impl: build ## Run the implementation binary. Usage: make run-impl [INPUT_FILE_PATH="file/path" | ARGS="--help"]
	make -C .. -f Makefile.rust -s run-raw BIN=cim-implementation ARGS="--input $(INPUT_FILE_PATH) $(ARGS)"

run-viz: build ## Run the visualization binary. Usage: make run-viz [INPUT_FILE_PATH="file/path" | OUTPUT_FILE_PATH="file/path"]
	make -C .. -f Makefile.rust -s run-raw BIN=visualization ARGS="--input $(INPUT_FILE_PATH) --output $(OUTPUT_FILE_PATH)"


.PHONY: build run-impl run-viz
