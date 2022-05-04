all: finite-state-machine-build

run: finite-state-machine-run

valgrind: finite-state-machine-valgrind

leaks: finite-state-machine-leaks

clean: finite-state-machine-clean


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


.PHONY: clean run valgrind leaks