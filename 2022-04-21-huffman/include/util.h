#pragma once

#include <cstdio>
#include <string>

/**
 * @brief A class that supports bit-level writing to file
 */
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

    // write a bit to file
    void writeBit(bool bit) const;

    // write a 8-bit char to file
    void write(uint8_t c) const;

    // return (total write bits) % 8
    uint8_t getCount() const;
};

/**
 * @brief A class that supports bit-level reading from file
 */
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

    // check if is End Of File
    // if so, the previous return cannot be used
    bool isEOF() const;

    // read a bit from file
    bool readBit() const;

    // read a char from file
    uint8_t readChar() const;
};