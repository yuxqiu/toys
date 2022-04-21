#include "huffman.h"
#include "util.h"
#include <iostream>
#include <queue>

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

    // 3. Compress Data
    Reader reader(infile);
    Writer writer(outfile);

    // 3.1 Output Trie
    Trie::writeTrie(*t, writer);

    // 3.2 Output compressed uint8_t
    uint8_t c = reader.readChar();
    while (!reader.isEOF())
    {
        const std::vector<bool> bits = table.lookup(c);
        for (const auto &b : bits)
        {
            writer.writeBit(b);
        }
        c = reader.readChar();
    }
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
    Reader reader(infile);
    Trie t = Trie::readTrie(reader);

    // 2. Decompress Data
    Writer writer(outfile);
    const Trie::Node *ptr = t.root.get();

    bool b = reader.readBit();
    while (!reader.isEOF())
    {
        while (!ptr->isLeaf() && !reader.isEOF())
        {
            updateNodePointer(ptr, b);
            b = reader.readBit();
        }

        // discard the padding bits
        if (!ptr->isLeaf())
        {
            break;
        }

        writer.write(ptr->c);
        ptr = t.root.get();
    }
}