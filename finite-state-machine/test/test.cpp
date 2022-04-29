#define CATCH_CONFIG_MAIN

#include "fsm.h"
#include "catch.hpp"

fsm::FiniteStateMachine *getDFA()
{
    return new fsm::DFA();
}

fsm::FiniteStateMachine *getNFA()
{
    return new fsm::NFA();
}

void shareCase1(fsm::FiniteStateMachine *(*f)(void))
{
    fsm::FiniteStateMachine *automata = f();
    automata->addState("q0", true);
    automata->addState("q1", false);

    automata->addTransition("q0", "q0", '0');
    automata->addTransition("q0", "q1", '1');
    automata->addTransition("q1", "q1", '0');
    automata->addTransition("q1", "q0", '1');

    automata->setStartState("q0");

    REQUIRE(automata->accept("0110"));
    REQUIRE(automata->accept("1001"));
    REQUIRE(!automata->accept("0001"));
    REQUIRE(!automata->accept("1000101"));

    delete automata;
}

void shareCase2(fsm::FiniteStateMachine *(*f)(void))
{
    fsm::FiniteStateMachine *automata = f();

    automata->addState("q0", false);
    automata->addState("q1", true);
    automata->addState("q2", false);
    automata->addState("q3", true);

    automata->addTransition("q0", "q1", 'a');
    automata->addTransition("q1", "q2", 'b');
    automata->addTransition("q2", "q3", 'b');

    REQUIRE(!automata->accept("ab"));
    REQUIRE(automata->accept("a"));
    REQUIRE(automata->accept("abb"));
    REQUIRE(!automata->accept("1000101"));

    delete automata;
}

void shareCase3(fsm::FiniteStateMachine *(*f)(void))
{
    fsm::FiniteStateMachine *automata = f();

    automata->addState("q0", true);
    automata->addState("q1", false);
    automata->addState("q2", false);

    automata->addTransition("q0", "q0", 'a');
    automata->addTransition("q0", "q1", 'b');
    automata->addTransition("q1", "q0", 'a');
    automata->addTransition("q1", "q2", 'b');
    automata->addSigmaTransition("q2", "q2");

    REQUIRE(!automata->accept("ab"));
    REQUIRE(automata->accept("a"));
    REQUIRE(automata->accept("ababa"));
    REQUIRE(!automata->accept("abbbbbaaa"));

    delete automata;
}

void shareCase4(fsm::FiniteStateMachine *(*f)(void))
{
    fsm::FiniteStateMachine *automata = f();

    automata->addState("q0", true);
    automata->addState("q1", false);
    automata->addState("q2", false);

    automata->addTransition("q0", "q1", '0');
    automata->addTransition("q1", "q2", '0');
    automata->addTransition("q2", "q0", '0');

    REQUIRE(!automata->accept("00"));
    REQUIRE(automata->accept("000"));
    REQUIRE(automata->accept("000000000000"));
    REQUIRE(!automata->accept("0010"));

    delete automata;
}

TEST_CASE("DFA Test Case 1: Accept strings with an even number of 1", "[DFA]")
{
    shareCase1(getDFA);
}

TEST_CASE("DFA Test Case 2: Accept strings 'a' and 'abb'", "[DFA]")
{
    shareCase2(getDFA);
}

TEST_CASE("DFA Test Case 3: Accept strings if all b's in the strings are immediately followed by an a", "[DFA]")
{
    shareCase3(getDFA);
}

TEST_CASE("DFA Test Case 4: Accept strings if they only contain 0, and their length are a multiple of 3", "[DFA]")
{
    shareCase4(getDFA);
}

TEST_CASE("NFA Test Case 1: DFA and NFA equivalence", "[NFA]")
{
    shareCase1(getNFA);
    shareCase2(getNFA);
    shareCase3(getNFA);
    shareCase4(getNFA);
}

TEST_CASE("NFA Test Case 2: Accept strings that contain 'hello'", "[NFA]")
{
    auto nfa = fsm::NFA();

    nfa.addState("q0", false);
    nfa.addState("q1", false);
    nfa.addState("q2", false);
    nfa.addState("q3", false);
    nfa.addState("q4", false);
    nfa.addState("q5", true);

    nfa.addSigmaTransition("q0", "q0");
    nfa.addTransition("q0", "q1", 'h');
    nfa.addTransition("q1", "q2", 'e');
    nfa.addTransition("q2", "q3", 'l');
    nfa.addTransition("q3", "q4", 'l');
    nfa.addTransition("q4", "q5", 'o');
    nfa.addSigmaTransition("q5", "q5");

    REQUIRE(nfa.accept("hello"));
    REQUIRE(nfa.accept("dahjdhakjdajkfbjkahellosdakjhdjahfdkjba"));
    REQUIRE(!nfa.accept("akfhbcaksfkhayugfbvscjknnc"));
    REQUIRE(!nfa.accept("hellao"));
}

TEST_CASE("NFA Test Case 3: Accept strings optionally start with 'baba' followed by an even number of b's", "[NFA]")
{
    auto nfa = fsm::NFA();

    nfa.addState("q0", false);
    nfa.addState("q1", false);
    nfa.addState("q2", false);
    nfa.addState("q3", false);
    nfa.addState("q4", true);
    nfa.addState("q5", false);

    nfa.addEpsilonTransition("q0", "q4");
    nfa.addTransition("q0", "q1", 'b');
    nfa.addTransition("q1", "q2", 'a');
    nfa.addTransition("q2", "q3", 'b');
    nfa.addTransition("q3", "q4", 'a');
    nfa.addTransition("q4", "q5", 'b');
    nfa.addTransition("q5", "q4", 'b');

    REQUIRE(nfa.accept(""));
    REQUIRE(nfa.accept("bababb"));
    REQUIRE(nfa.accept("bb"));
    REQUIRE(nfa.accept("bbbb"));
    REQUIRE(!nfa.accept("b"));
    REQUIRE(!nfa.accept("ab"));
    REQUIRE(!nfa.accept("babab"));
    REQUIRE(!nfa.accept("babb"));
}