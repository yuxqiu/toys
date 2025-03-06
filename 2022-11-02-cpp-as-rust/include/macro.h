#pragma once

#include <cassert>
#include <cstdio>
#include <cstdlib>

#define line __LINE__
#define file __FILE__
#define stringify(s) #s
#define expand_and_stringify(s) stringify(s)

// Print - could be replaced by std::format in C++20
#define print(...) fprintf(stdout, __VA_ARGS__)
#define println(...)              \
    fprintf(stdout, __VA_ARGS__); \
    fprintf(stdout, "\n")
#define eprint(...) fprintf(stderr, __VA_ARGS__)
#define eprintln(...)             \
    fprintf(stderr, __VA_ARGS__); \
    fprintf(stderr, "\n")

// Assertion
#define assert_eq(left, right) assert(left == right)

// Limited functionality because of std::exit
#define panic                                                     \
    eprintln("Panicked at " file ":" expand_and_stringify(line)); \
    std::exit(EXIT_FAILURE)
#define unimplemented                                     \
    eprint("Unimplemented function ");                    \
    eprint(__func__);                                     \
    eprintln(" at " file ":" expand_and_stringify(line)); \
    std::exit(EXIT_FAILURE)
#define todo                                              \
    eprint("Todo function ");                             \
    eprint(__func__);                                     \
    eprintln(" at " file ":" expand_and_stringify(line)); \
    std::exit(EXIT_FAILURE)
