#include "fsm.h"

using namespace fsm;

DFA::Node::Node(const std::string &name, bool isAcceptingState)
    : name(name),
      is_accepting_state(isAcceptingState)
{
}

void DFA::Node::addTransition(Node &n, char transition)
{
    transitions.insert({int(transition), n});
    transitions.erase(SIGMA);
}

void DFA::Node::addSigmaTransition(Node &n)
{
    transitions.clear();
    transitions.insert({SIGMA, n});
}

DFA::Node *DFA::Node::getNext(char transition) const
{
    if (transitions.find(SIGMA) != transitions.end())
    {
        return &transitions.at(SIGMA);
    }
    if (transitions.find(int(transition)) != transitions.end())
    {
        return &transitions.at(transition);
    }
    return nullptr;
}

bool DFA::Node::isAcceptingState() const { return is_accepting_state; }

void DFA::addState(const std::string &name, bool isAcceptingState)
{
    check1_non(name);

    nodes.insert({name, Node(name, isAcceptingState)});

    if (!startState)
    {
        startState = &nodes.at(name);
    }
}

void DFA::addTransition(const std::string &from, const std::string &to, char transition)
{
    check2(from, to);

    nodes.at(from).addTransition(nodes.at(to), transition);
}

void DFA::addSigmaTransition(const std::string &from, const std::string &to)
{
    check2(from, to);

    nodes.at(from).addSigmaTransition(nodes.at(to));
}

void DFA::setStartState(const std::string &name)
{
    check1_in(name);

    startState = &nodes.at(name);
}

bool DFA::accept(const std::string &s) const
{
    const Node *st = startState;
    size_t index = 0;

    while (st != nullptr && index < s.length())
    {
        st = st->getNext(s[index]);
        index += 1;
    }

    return st != nullptr && st->isAcceptingState();
}

bool DFA::Node::operator==(const DFA::Node &other) const
{
    return name.compare(other.name) == 0;
}

NFA::Node::Node(const std::string &name, bool isAcceptingState)
    : name(name),
      is_accepting_state(isAcceptingState)
{
}

void NFA::Node::addTransition(Node &n, char transition)
{
    if (transitions.find(transition) == transitions.end())
    {
        std::unordered_map<std::string, Node &> s;
        transitions.insert({transition, s});
    }
    transitions.at(transition).insert({n.name, n});
}

void NFA::Node::addSigmaTransition(Node &n)
{
    if (transitions.find(SIGMA) == transitions.end())
    {
        std::unordered_map<std::string, Node &> s;
        transitions.insert({SIGMA, s});
    }
    transitions.at(SIGMA).insert({n.name, n});
}

void NFA::Node::addEpsilonTransition(Node &n)
{
    if (transitions.find(EPSILON) == transitions.end())
    {
        std::unordered_map<std::string, Node &> s;
        transitions.insert({EPSILON, s});
    }
    transitions.at(EPSILON).insert({n.name, n});
}

std::unordered_map<std::string, NFA::Node &> NFA::Node::getNext(char transition) const
{
    std::unordered_map<std::string, Node &> ret;

    if (transitions.find(SIGMA) != transitions.end())
    {
        for (auto it = transitions.at(SIGMA).begin(); it != transitions.at(SIGMA).end(); ++it)
        {
            ret.insert(*it);
        }
    }

    if (transitions.find(transition) == transitions.end())
    {
        return ret;
    }

    for (auto it = transitions.at(transition).begin(); it != transitions.at(transition).end(); ++it)
    {
        ret.insert(*it);
    }

    return ret;
}

bool NFA::Node::isAcceptingState() const { return is_accepting_state; }

bool NFA::Node::containsEpsilonTransition() const { return transitions.find(EPSILON) != transitions.end(); }

void NFA::addState(const std::string &name, bool isAcceptingState)
{
    check1_non(name);

    nodes.insert({name, Node(name, isAcceptingState)});

    if (!startState)
    {
        startState = &nodes.at(name);
    }
}

void NFA::addTransition(const std::string &from, const std::string &to, char transition)
{
    check2(from, to);

    nodes.at(from).addTransition(nodes.at(to), transition);
}

void NFA::addSigmaTransition(const std::string &from, const std::string &to)
{
    check2(from, to);

    nodes.at(from).addSigmaTransition(nodes.at(to));
}

void NFA::addEpsilonTransition(const std::string &from, const std::string &to)
{
    check2(from, to);

    nodes.at(from).addEpsilonTransition(nodes.at(to));
}

void NFA::setStartState(const std::string &name)
{
    check1_in(name);

    startState = &nodes.at(name);
}

void NFA::dfs(const Node &n, std::unordered_map<std::string, Node &> &states) const
{
    if (n.containsEpsilonTransition())
    {
        auto newNodes = n.getNext(EPSILON);
        for (auto it = newNodes.begin(); it != newNodes.end(); ++it)
        {
            if (states.find(it->first) == states.end())
            {
                states.insert(*it);
                dfs(it->second, states);
            }
        }
    }
}

void NFA::closure(std::unordered_map<std::string, Node &> &states) const
{
    std::unordered_map<std::string, Node &> temp = states;

    for (auto it = temp.begin(); it != temp.end(); ++it)
    {
        dfs(it->second, states);
    }
}

bool NFA::accept(const std::string &s) const
{
    std::unordered_map<std::string, Node &> states;
    states.insert({startState->name, *startState});
    closure(states);

    for (size_t i = 0; i < s.length() && !states.empty(); ++i)
    {
        std::unordered_map<std::string, Node &> newStates;

        for (auto it = states.begin(); it != states.end(); ++it)
        {
            std::unordered_map<std::string, Node &> newNodesFromIter = it->second.getNext(s[i]);
            for (auto itNewNode = newNodesFromIter.begin(); itNewNode != newNodesFromIter.end(); ++itNewNode)
            {
                newStates.insert(*itNewNode);
            }
        }

        closure(newStates);
        states = std::unordered_map<std::string, Node &>(std::move(newStates));
    }

    for (auto it = states.begin(); it != states.end(); ++it)
    {
        if (it->second.isAcceptingState())
        {
            return true;
        }
    }

    return false;
}

bool NFA::Node::operator==(const NFA::Node &other) const
{
    return name.compare(other.name) == 0;
}