#pragma once

#include <cstdio>
#include <string>

class Writer
{
private:
    FILE *file;
    mutable uint8_t buffer;
    mutable uint8_t count;

public:
    Writer(const std::string &filename);
    ~Writer();
    Writer(const Writer &other) = delete;
    Writer &operator=(const Writer &other) = delete;

    void writeBit(bool bit) const;
    void write(uint8_t c) const;
    uint8_t getCount() const;
};

class Reader
{
private:
    FILE *file;
    mutable uint8_t buffer;
    mutable uint8_t count;

public:
    Reader(const std::string &filename);
    ~Reader();
    Reader(const Reader &other) = delete;
    Reader &operator=(const Reader &other) = delete;

    bool isEOF() const;
    bool readBit() const;
    uint8_t readChar() const;
};