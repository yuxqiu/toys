#include "huffman.h"
#include <iostream>

int main(int argc, char **argv)
{
    if (argc != 3)
    {
        std::cerr << "Usage: ./main [input] [output]\n";
        return 1;
    }

    std::string infile(argv[1]), outfile(argv[2]);
    std::cout << "Please select the mode:\n 1. compress\n 2. decompress" << std::endl;
    int c;
    std::cin >> c;

    switch (c)
    {
    case 1:
        Huffman::compress(infile, outfile);
        break;
    case 2:
        Huffman::decompress(infile, outfile);
        break;
    default:
        throw std::runtime_error("Invalid mode...");
        break;
    }

    return 0;
}
