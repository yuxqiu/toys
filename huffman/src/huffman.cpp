#include "huffman.h"
#include "util.h"
#include <iostream>
#include <queue>

std::unique_ptr<Trie::Node> Huffman::_readTrie(const Reader &reader)
{
    if (reader.readBit())
    {
        if (reader.isEOF())
        {
            throw std::runtime_error("Invalid File Format");
        }

        std::unique_ptr<Trie::Node> temp = std::make_unique<Trie::Node>();

        temp->c = reader.readChar();
        if (reader.isEOF())
        {
            throw std::runtime_error("Invalid File Format");
        }

        return temp;
    }

    std::unique_ptr<Trie::Node> left = _readTrie(reader);
    std::unique_ptr<Trie::Node> right = _readTrie(reader);

    std::unique_ptr<Trie::Node> root = std::make_unique<Trie::Node>();
    root->left = std::move(left);
    root->right = std::move(right);
    return root;
}

Trie Huffman::readTrie(const Reader &reader)
{
    Trie t(_readTrie(reader));
    if (t.root->isLeaf())
    {
        throw std::runtime_error("Invalid File Format");
    }
    return t;
}

void Huffman::_writeTrie(const std::unique_ptr<Trie::Node> &ptr, const Writer &writer){
    if (ptr->isLeaf())
    {
        writer.writeBit(true);
        writer.write(ptr->c);
        return;
    }

    writer.writeBit(false);

    _writeTrie(ptr->left, writer);
    _writeTrie(ptr->right, writer);
}

void Huffman::saveToFile(const Data & data, const std::string& outfile){
    Writer writer(outfile);
    _writeTrie(data.t.root, writer);

    const std::vector<bool>& v = data.data;
    uint8_t modulus = (writer.getCount() + v.size()) % 8;
    if(modulus != 0){
        modulus = 8 - modulus;
    }
    writer.write(modulus);

    for (const auto &b : v)
    {
        writer.writeBit(b);
    }
}

Data Huffman::loadFromFile(const std::string &infile){
    Reader reader(infile);
    Trie t = readTrie(reader);
    uint8_t modulus = reader.readChar();

    std::vector<bool> v;
    v.reserve(VEC_SIZE);

    bool b = reader.readBit();
    while(!reader.isEOF()){
        v.push_back(b);
        b = reader.readBit();
    }
    for(uint8_t i = 0; i < modulus; ++i){
        v.pop_back();
    }

    return {std::move(t), std::move(v)};
}

void Huffman::compress(const std::string &infile, const std::string &outfile)
{
    size_t count[CHAR_SIZE] = {};

    // 1. Count Frequency
    {
        Reader reader(infile);
        uint8_t c = reader.readChar();
        while (!reader.isEOF())
        {
            ++count[c];
            c = reader.readChar();
        }
    }

    // 2. Build Trie and Table
    std::shared_ptr<Trie> t = buildTrie(count);
    LookupTable table(*t);

    // 3. Compress
    Reader reader(infile);
    Data data = {std::move(*t), std::vector<bool>()};
    data.data.reserve(VEC_SIZE);

    // 3.1 Load into memory
    uint8_t c = reader.readChar();
    while (!reader.isEOF())
    {
        const std::vector<bool>& bits = table.lookup(c);
        std::copy(bits.cbegin(), bits.cend(), std::back_inserter(data.data));
        c = reader.readChar();
    }

    // 3.2 Save to file
    saveToFile(data, outfile);
}

std::shared_ptr<Trie> Huffman::buildTrie(size_t count[CHAR_SIZE])
{
    auto compare = [](const std::shared_ptr<Trie> &lhs, const std::shared_ptr<Trie> &rhs)
    {
        return *rhs < *lhs;
    };
    std::priority_queue<std::shared_ptr<Trie>, std::vector<std::shared_ptr<Trie>>, decltype(compare)> pq(compare);

    for (size_t i = 0; i < CHAR_SIZE; ++i)
    {
        if (count[i] != 0)
        {
            pq.push(std::make_shared<Trie>(i, count[i]));
        }
    }

    while (pq.size() > 1)
    {
        std::shared_ptr<Trie> x = pq.top();
        pq.pop();
        std::shared_ptr<Trie> y = pq.top();
        pq.pop();

        x->merge(std::move(*y));
        pq.push(x);
    }

    // Only 1 character in the text
    std::shared_ptr<Trie> t = pq.top();
    if (t->root->isLeaf())
    {
        std::unique_ptr<Trie::Node> root = std::make_unique<Trie::Node>();
        root->left = std::move(t->root);
        root->right = std::make_unique<Trie::Node>();
        root->right->c = ((unsigned int)(root->left->c) + 1) & 0xff;
        t->root = std::move(root);
    }

    return t;
}

void Huffman::updateNodePointer(Trie::Node const *&ptr, bool b)
{
    if (b)
    {
        if (!(ptr->right))
        {
            throw std::runtime_error("Invalid File Format");
        }
        ptr = ptr->right.get();
    }
    else
    {
        if (!(ptr->left))
        {
            throw std::runtime_error("Invalid File Format");
        }
        ptr = ptr->left.get();
    }
}

void Huffman::decompress(const std::string &infile, const std::string &outfile)
{
    Data data = loadFromFile(infile);

    // 2. Decompress Data
    Writer writer(outfile);
    const Trie::Node *ptr = data.t.root.get();

    for (size_t i = 0; i < data.data.size(); ++i)
    {
        updateNodePointer(ptr, data.data[i]);
        if (ptr->isLeaf())
        {
            writer.write(ptr->c);
            ptr = data.t.root.get();
        }
    }
}