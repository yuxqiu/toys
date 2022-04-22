#pragma once

#include <cstddef>
#include <map>
#include <memory>
#include <vector>

/**
 * @brief A trie data structure
 */
class Trie
{
private:
    struct Node
    {
        std::unique_ptr<Node> left, right;
        uint8_t c;

        Node(uint8_t c, std::unique_ptr<Node> &&left, std::unique_ptr<Node> &&right)
            : left(std::move(left)), right(std::move(right)), c(c) {}

        Node(uint8_t c) : c(c) {}

        bool isLeaf() const
        {
            return !(left) && !(right);
        }
    };

    std::unique_ptr<Node> root;
    size_t frequency;

public:
    Trie(uint8_t c, size_t frequency);
    Trie(std::unique_ptr<Node> &&root);
    Trie(const Trie &other) = delete;
    Trie &operator=(const Trie &other) = delete;
    Trie(Trie &&other);
    Trie &operator=(Trie &&other);

    // merge two tries into one by connecting two tries with a new root node
    void merge(Trie &&other);

    friend bool operator<(const Trie &lhs, const Trie &rhs);
    friend class LookupTable;
    friend class Huffman;
};

bool operator<(const Trie &lhs, const Trie &rhs);

/**
 * @brief A class that builds a lookup table to associate a char with its Huffman Encoding
 */
class LookupTable
{
private:
    std::map<uint8_t, std::vector<bool>> table;

private:
    void buildTable(const std::unique_ptr<Trie::Node> &ptr, std::vector<bool> &bits);

public:
    LookupTable(const Trie &t);

    // look up the corresponding Huffman Encoding of a char
    const std::vector<bool> &lookup(uint8_t c) const;
};