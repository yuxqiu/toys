#include "trie.h"

Trie::Trie(uint8_t c, size_t frequency) : frequency(frequency)
{
    root = std::make_unique<Node>();
    root->c = c;
}

Trie::Trie(std::unique_ptr<Node> &&root) : root(std::move(root)), frequency(0)
{
}

Trie::Trie(Trie &&other) : root(std::move(other.root)), frequency(other.frequency)
{
}

Trie &Trie::operator=(Trie &&other)
{
    if (this != &other)
    {
        root = std::move(other.root);
        frequency = other.frequency;
    }
    return *this;
}

void Trie::merge(Trie &&other)
{
    std::unique_ptr<Node> temp = std::make_unique<Node>();
    temp->left = std::move(root);
    temp->right = std::move(other.root);
    root = std::move(temp);
    frequency += other.frequency;
}

void Trie::_writeTrie(const std::unique_ptr<Node> &ptr, const Writer &writer)
{
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

void Trie::writeTrie(const Trie &t, const Writer &writer)
{
    _writeTrie(t.root, writer);
}

std::unique_ptr<Trie::Node> Trie::_readTrie(const Reader &reader)
{
    if (reader.readBit())
    {
        if (reader.isEOF())
        {
            throw std::runtime_error("Invalid File Format");
        }

        std::unique_ptr<Node> temp = std::make_unique<Node>();

        temp->c = reader.readChar();
        if (reader.isEOF())
        {
            throw std::runtime_error("Invalid File Format");
        }

        return temp;
    }

    std::unique_ptr<Node> left = _readTrie(reader);
    std::unique_ptr<Node> right = _readTrie(reader);

    std::unique_ptr<Node> root = std::make_unique<Node>();
    root->left = std::move(left);
    root->right = std::move(right);
    return root;
}

Trie Trie::readTrie(const Reader &reader)
{
    Trie t(_readTrie(reader));

    // root can never be leaf
    if (t.root->isLeaf())
    {
        throw std::runtime_error("Invalid File Format");
    }

    return t;
}

bool operator<(const Trie &lhs, const Trie &rhs)
{
    return lhs.frequency < rhs.frequency;
}

void LookupTable::buildTable(const std::unique_ptr<Trie::Node> &ptr, std::vector<bool> &bits)
{
    if (ptr->isLeaf())
    {
        table[ptr->c] = bits;
        return;
    }

    bits.push_back(false);
    buildTable(ptr->left, bits);
    bits.pop_back();

    bits.push_back(true);
    buildTable(ptr->right, bits);
    bits.pop_back();
}

LookupTable::LookupTable(const Trie &t)
{
    std::vector<bool> bits;
    buildTable(t.root, bits);
}

const std::vector<bool> &LookupTable::lookup(uint8_t c) const
{
    auto p = table.find(c);
    if (p == table.end())
    {
        throw std::runtime_error("uint8_t doesn't exist in the table");
    }
    return p->second;
}