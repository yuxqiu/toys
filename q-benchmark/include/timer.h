#pragma once

#include <chrono>

class Timer
{
public:
    Timer() : m_start(std::chrono::high_resolution_clock::now())
    {
    }

    double stop() const;

private:
    std::chrono::time_point<std::chrono::high_resolution_clock> m_start;
};