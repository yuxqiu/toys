# Finite State Machines

finite-state machine is a C++ implementation of some Finite-state Machines, including DFA, NFA that support the standard operations.

## Installation and Compilation

To use this project, first clone the repo on your device.

Then, add the following line to your source file:

```c++
#include "fsm.hpp"
```

## Usage

You can create a finite state machine via the following methods:

```c++
// This will create a DFA.
fsm::DFA dfa;

// This will create a NFA.
fsm::NFA nfa;
```

With the finite-state machine object, you can use the following methods:

```c++
// Add state
// the first added state is by default set as the start state
bool is_an_accepting_state = true;
dfa.addState("state_name", is_an_accepting_state);
nfa.addState("state_name", is_an_accepting_state);

// This sets the state with "state_name" as start state
dfa.setStartState("state_name");
nfa.setStartState("state_name");

// This adds a transition q0 -> q0 that uses char '0'
dfa.addTransition("q0", "q0", '0');
nfa.addTransition("q0", "q0", '0');

// This adds a \Sigma transition q0 -> q0
dfa.addSigmaTransition("q0", "q0");
nfa.addSigmaTransition("q0", "q0");

// This adds a \epsilon transition q0 -> q0
// Only NFA supports this operation
nfa.addEpsilonTransition("q0", "q0");

// This checks whether the machine accepts "str"
dfa.accept("str");
nfa.accept("str");
```

## Test

The library has been tested by using `Catch2` (C++ part).

Before running C++ test, make sure you put `catch.hpp` in include folder, and run `make` to generate the test executable.

## New Features

- [ ] Design Improvements
- [ ] Support for Non-Deterministic PDA
- [ ] Support for Turing Machine