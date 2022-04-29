#pragma once

#include <cassert>
#include <string>
#include <unordered_map>

#define check1_non(name) (assert(nodes.find(name) == nodes.end()))
#define check1_in(name) (assert(nodes.find(name) != nodes.end()))
#define check2(from, to) (assert(nodes.find(from) != nodes.end() && nodes.find(to) != nodes.end()))

namespace fsm
{
    enum Transition
    {
        SIGMA = -1,
        EPSILON = -2
    };

    class FiniteStateMachine
    {
    public:
        virtual ~FiniteStateMachine() = default;
        virtual void addState(const std::string &name, bool is_accepting_state) = 0;
        virtual void addSigmaTransition(const std::string &from, const std::string &to) = 0;
        virtual void addTransition(const std::string &from, const std::string &to, char transition) = 0;
        virtual void setStartState(const std::string &name) = 0;
        virtual bool accept(const std::string &s) const = 0;
    };

    class DFA : public FiniteStateMachine
    {
    public:
        class Node
        {
        private:
            std::string name;
            bool is_accepting_state;
            std::unordered_map<int, Node &> transitions;

        private:
            Node(const std::string &name, bool isAcceptingState);
            void addTransition(Node &n, char transition);
            void addSigmaTransition(Node &n);
            Node *getNext(char transition) const;
            bool isAcceptingState() const;

            bool operator==(const Node &other) const;

            friend class DFA;
        };

    private:
        std::unordered_map<std::string, Node> nodes;
        Node *startState = nullptr;

    public:
        DFA() = default;
        ~DFA() = default;

        void addState(const std::string &name, bool is_accepting_state) override;
        void addSigmaTransition(const std::string &from, const std::string &to) override;
        void addTransition(const std::string &from, const std::string &to, char transition) override;
        void setStartState(const std::string &name) override;
        bool accept(const std::string &s) const override;
    };

    class NFA : public FiniteStateMachine
    {
    public:
        class Node
        {
        private:
            std::string name;
            bool is_accepting_state;
            std::unordered_map<int, std::unordered_map<std::string, Node &>> transitions;

        private:
            Node(const std::string &name, bool isAcceptingState);
            void addTransition(Node &n, char transition);
            void addSigmaTransition(Node &n);
            void addEpsilonTransition(Node &n);
            bool containsEpsilonTransition() const;
            std::unordered_map<std::string, Node &> getNext(char transition) const;
            bool isAcceptingState() const;

            bool operator==(const Node &other) const;

            friend class NFA;
        };

    private:
        std::unordered_map<std::string, Node> nodes;
        Node *startState = nullptr;

    public:
        NFA() = default;
        ~NFA() = default;

        void addState(const std::string &name, bool is_accepting_state) override;
        void addSigmaTransition(const std::string &from, const std::string &to) override;
        void addTransition(const std::string &from, const std::string &to, char transition) override;
        void addEpsilonTransition(const std::string &from, const std::string &to);
        void setStartState(const std::string &name) override;
        bool accept(const std::string &s) const override;

    private:
        void dfs(const Node &n, std::unordered_map<std::string, Node &> &states) const;
        void closure(std::unordered_map<std::string, Node &> &states) const;
    };
};
