ARGS :=

build:
	make -C .. -f Makefile.rust -s build PACKAGE=tp4a

run-all: build
	make -s -C .. -f Makefile.rust -s run-raw BIN="tp4a" ARGS="verlet" > data/verlet.csv
	make -s -C .. -f Makefile.rust -s run-raw BIN="tp4a" ARGS="gear" > data/gear.csv
	make -s -C .. -f Makefile.rust -s run-raw BIN="tp4a" ARGS="beeman" > data/beeman.csv
	make -s -C .. -f Makefile.rust -s run-raw BIN="tp4a" ARGS="analytic" > data/analytic.csv

.PHONY: build run-all
