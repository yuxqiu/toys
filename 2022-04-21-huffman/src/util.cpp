#include "util.h"

Writer::Writer(const std::string &filename) : buffer(0), count(0)
{
    file = fopen(filename.c_str(), "wb");
    if (!file)
    {
        throw std::runtime_error("Failed to open the file " + filename);
    }
}

Writer::~Writer()
{
    if (count != 0)
    {
        count = 8;
        writeBit(false);
    }
    fclose(file);
}

void Writer::writeBit(bool bit) const
{
    if (count == 8)
    {
        if (fwrite(&buffer, sizeof(uint8_t), 1, file) != 1)
        {
            throw std::runtime_error("Failed to write to the file");
        }
        count = 0;
        buffer = 0;
    }

    if (bit)
    {
        buffer |= (1 << (7 - count));
    }
    ++count;
}

void Writer::write(uint8_t c) const
{
    for (uint8_t i = 0; i < 8; ++i)
    {
        writeBit((c >> (7 - i)) & 1);
    }
}

uint8_t Writer::getCount() const{
    return count;
}

Reader::Reader(const std::string &filename) : buffer(0), count(8)
{
    file = fopen(filename.c_str(), "rb");
    if (!file)
    {
        throw std::runtime_error("Failed to open the file " + filename);
    }
}

Reader::~Reader()
{
    fclose(file);
}

bool Reader::isEOF() const
{
    return feof(file);
}

bool Reader::readBit() const
{
    if (count == 8)
    {
        if (fread(&buffer, sizeof(uint8_t), 1, file) != 1 && ferror(file))
        {
            throw std::runtime_error("Failed to read from the file");
        }
        count = 0;
    }
    ++count;

    return ((buffer >> (8 - count)) & 1);
}

uint8_t Reader::readChar() const
{
    uint8_t c = 0;
    for (uint8_t i = 0; i < 8; ++i)
    {
        if (readBit())
        {
            c |= (1 << (7 - i));
        }
    }
    return c;
}