ARGS :=
DELTA_T_EXPONENT := 5

build:
	make -C .. -f Makefile.rust -s build PACKAGE=tp4a

run-all: build
	for method in verlet gear beeman analytic; do \
		make -s -C .. -f Makefile.rust -s run-raw BIN="tp4a" ARGS="--delta-t-exponent ${DELTA_T_EXPONENT} $${method}" > "data/$${method}.csv"; \
	done

run-mse: build
	echo method,dt,mse > data/mse.csv
	for method in verlet gear beeman; do \
		for exponent in 6 5 4 3 2; do \
			mse=$$(make -s -C .. -f Makefile.rust -s run-raw BIN="tp4a" ARGS="--delta-t-exponent=$${exponent} $${method}" 2>&1 >/dev/null); \
			echo $${method},10e-$${exponent},$${mse} >> data/mse.csv; \
		done \
	done

.PHONY: build run-all
