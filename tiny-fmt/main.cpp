#include "fmt.h"

int main(void){
    // Parsing
    fmt::format("x:{}, y:{}\n", 1, 2.5);
    fmt::format("escaped text: {{ hello escaped! }}\n", true);

    // Plain text
    fmt::format("escaped text: {{ hello escaped! }}\n");
    fmt::format("hello world!\n");

    // No enough args
    fmt::format("No enough args: {}{}\n", 1);

    // Too many args
    fmt::format("Too many args: {}\n", 1, 2, 3, 4);
}