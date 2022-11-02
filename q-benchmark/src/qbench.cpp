#include "runner.h"
#include <cstring>
#include <memory>
#include <random>
#include <unistd.h>

#define DELIMITER " \t\r\a\n"
#define BUFFER_SIZE 1024

/*
    Parse the options into char** array
*/
std::unique_ptr<char *[]> parseOptInProgram(char *program)
{
    size_t size = BUFFER_SIZE;
    size_t cur = 0;

    std::unique_ptr<char *[]> args = std::make_unique<char *[]>(BUFFER_SIZE);

    char *token = strtok(program, DELIMITER);
    while (token != nullptr)
    {
        if (cur == size - 1)
        {
            std::unique_ptr<char *[]> tmp = std::make_unique<char *[]>(size * 2);
            std::copy(args.get(), args.get() + size, tmp.get());
            args = std::move(tmp);
            size *= 2;
        }
        args[cur] = token;
        ++cur;
        token = strtok(NULL, DELIMITER);
    }
    args[cur] = NULL;

    if (cur == 0)
    {
        return std::unique_ptr<char *[]>();
    }

    return args;
}

int main(int argc, char **argv)
{
    size_t warmupTimes = 3;
    size_t MIN = 100;
    size_t MAX = 1000;

    // parse options
    int option;
    while ((option = getopt(argc, argv, "w:l:h:")) != -1)
    {
        switch (option)
        {
        case 'w':
            warmupTimes = std::stoul(optarg);
            break;
        case 'l':
            if (!optarg)
                throw std::invalid_argument("-l needs to be provided with an option");
            MIN = std::stoul(optarg);
            break;
        case 'h':
            if (!optarg)
                throw std::invalid_argument("-h needs to be provided with an option");
            MAX = std::stoul(optarg);
            break;
        default:
            throw std::invalid_argument("Unknown argument(s)");
        }
    }

    // ensure the options are valid
    MIN = MIN <= 0 ? 1 : MIN;
    MAX = MAX < MIN ? MIN : MAX;

    // Random Number Generator
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> distrib(MIN, MAX);

    // run benchmark for a list of programs
    for (int i = optind; i < argc; ++i)
    {
        Runner runner(parseOptInProgram(argv[i]), distrib(gen), warmupTimes);
        runner.run();
        if (runner.hasTestRunSuccessfully())
            runner.display();
    }

    return EXIT_SUCCESS;
}