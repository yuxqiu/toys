all: finite-state-machine-build my-matrix-build

run: finite-state-machine-run my-matrix-run

valgrind: finite-state-machine-valgrind my-matrix-valgrind

leaks: finite-state-machine-leaks my-matrix-leaks

clean: finite-state-machine-clean my-matrix-clean


# =============================================== FSM
finite-state-machine-build:
	make --directory finite-state-machine

finite-state-machine-run:
	make run --directory finite-state-machine

finite-state-machine-valgrind:
	make valgrind --directory finite-state-machine

finite-state-machine-leaks:
	make leaks --directory finite-state-machine

finite-state-machine-clean:
	make clean --directory finite-state-machine
# =============================================== FSM

# =============================================== MyMatrix
my-matrix-build:
	make --directory MyMatrix

my-matrix-run:
	make run --directory MyMatrix

my-matrix-valgrind:
	make valgrind --directory MyMatrix

my-matrix-leaks:
	make leaks --directory MyMatrix

my-matrix-clean:
	make clean --directory MyMatrix
# =============================================== MyMatrix


.PHONY: clean run valgrind leaks