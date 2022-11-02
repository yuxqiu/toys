#include "runner.h"
#include "timer.h"
#include <cmath>
#include <iostream>
#include <sys/wait.h>
#include <unistd.h>

void Runner::run()
{
    if (!args)
    {
        std::cerr << "None was supplied for commands" << std::endl;
        return;
    }

    if (warmup())
    {
        hasRunSuccess = runTest();
    }
}

bool Runner::hasTestRunSuccessfully() const
{
    return hasRunSuccess;
}

void Runner::display() const
{
    if (!hasRunSuccess)
    {
        std::cerr << "Test failed to run" << std::endl;
        return;
    }

    double mean = 0, std = 0, min_v = tests[0], max_v = tests[0];

    for (std::size_t i = 0; i < numRuns; ++i)
    {
        mean += tests[i];
        min_v = tests[i] < min_v ? tests[i] : min_v;
        max_v = tests[i] > max_v ? tests[i] : max_v;
    }
    mean /= numRuns;

    for (std::size_t i = 0; i < numRuns; ++i)
        std += pow(tests[i] - mean, 2);
    std = sqrt(std / numRuns);

    std::cout << STYLE_BOLD << "Benchmark: " << args[0] << " ("
              << BLUE << numRuns << RESET << " runs)"
              << STYLE_NO_BOLD << std::endl;

    std::cout << "  Time: " << GREEN << mean << " ms" << RESET << "(mean) ± "
              << PURPLE << std << " ms" << RESET << "(std)" << std::endl;

    std::cout << "  Range: " << GREEN << min_v << " ms" << RESET << "(min) … "
              << PURPLE << max_v << " ms" << RESET << "(max)" << std::endl;

    std::cout << std::endl;
}

bool Runner::launch() const
{
    pid_t pid;
    int status;

    // fork a child process
    pid = fork();

    if (pid == 0)
    {
        // rewire the file descriptor
        FILE *f = fopen("/dev/null", "w");
        int fd = fileno(f);
        dup2(fd, 1);
        dup2(fd, 2);
        close(fd);

        // run the benchmark program with args
        if (execvp(args[0], args.get()) == -1)
        {
            exit(EXIT_FAILURE);
        }
    }
    else if (pid < 0)
    {
        throw std::runtime_error("Failed to fork() the program");
    }
    else
    {
        do
        {
            waitpid(pid, &status, WUNTRACED);
        } while (!WIFEXITED(status) && !WIFSIGNALED(status));

        if (WEXITSTATUS(status) != EXIT_SUCCESS)
            std::cerr << "Failed to run " << args[0] << std::endl;
        return WEXITSTATUS(status) == EXIT_SUCCESS;
    }

    return false;
}

bool Runner::warmup() const
{
    for (std::size_t i = 0; i < warmupTimes; ++i)
    {
        if (!launch())
        {
            return false;
        }
    }
    return true;
}

bool Runner::runTest()
{
    for (std::size_t i = 0; i < numRuns; ++i)
    {
        Timer t;
        bool success = launch();
        tests[i] = t.stop();

        if (!success)
        {
            return false;
        }
    }
    return true;
}