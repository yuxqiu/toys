#include "timer.h"

double Timer::stop() const
{
    auto m_end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(m_end - m_start);
    return duration.count() * 0.001;
}