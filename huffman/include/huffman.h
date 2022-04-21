#pragma once

#include "trie.h"
#include <string>

constexpr size_t CHAR_SIZE = 256;

class Huffman
{
private:
    static std::shared_ptr<Trie> buildTrie(size_t array[CHAR_SIZE]);
    static void updateNodePointer(Trie::Node const *&ptr, bool b);
public:
    static void compress(const std::string &infile, const std::string &outfile);
    static void decompress(const std::string &infile, const std::string &outfile);

    Huffman() = delete;
};