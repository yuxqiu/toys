#pragma once

#include <memory>

#define STYLE_BOLD "\033[1m"
#define STYLE_NO_BOLD "\033[22m"
#define GREEN "\033[32m"
#define PURPLE "\033[35m"
#define BLUE "\x1b[36m"
#define RESET "\033[0m"

class Runner
{
public:
    Runner(std::unique_ptr<char *[]> &&args, std::size_t numRuns, std::size_t warmupTimes)
        : args(std::move(args)), numRuns(numRuns), warmupTimes(warmupTimes), tests(std::make_unique<double[]>(numRuns))
    {
    }

    ~Runner()
    {
    }

    // wrapper of warmup and runTest
    void run();

    bool hasTestRunSuccessfully() const;

    void display() const;

private:
    bool launch() const;

    // run warmup to minimize the effect of cache
    bool warmup() const;

    bool runTest();

private:
    // Program Info
    const std::unique_ptr<char *[]> args;
    const std::size_t numRuns;
    const size_t warmupTimes;

    // Test Result
    const std::unique_ptr<double[]> tests;

    // Flag
    bool hasRunSuccess = false;
};