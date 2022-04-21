#pragma once

#include "trie.h"
#include "util.h"
#include <string>

constexpr size_t CHAR_SIZE = 256;
constexpr size_t VEC_SIZE = 1024;

struct Data
{
    Trie t;
    std::vector<bool> data;
};

class Huffman
{
private:
    // build a trie from an array
    static std::shared_ptr<Trie> buildTrie(size_t array[CHAR_SIZE]);

    // update a Trie::Node pointer based on the boolean value
    static void updateNodePointer(Trie::Node const *&ptr, bool b);

    // helper function to read a Trie::Node pointer from file
    static std::unique_ptr<Trie::Node> _readTrie(const Reader &reader);
    static void _writeTrie(const std::unique_ptr<Trie::Node> &ptr, const Writer &writer);

    // use _readTrie helper to read a Trie::Node pointer and wrap it into a trie
    static Trie readTrie(const Reader &reader);

    // save a Data to file
    static void saveToFile(const Data & data, const std::string& outfile);

    // load a Data from file
    static Data loadFromFile(const std::string &infile);

public:
    // compress a file and output to another file
    static void compress(const std::string &infile, const std::string &outfile);

    // decompress a file and output to another file
    static void decompress(const std::string &infile, const std::string &outfile);

    Huffman() = delete;
};