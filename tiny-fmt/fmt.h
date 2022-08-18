#pragma once

#include <cstddef>
#include <iostream>
#include <ostream>
#include <string_view>
#include <utility>

class fmt {
private:
  static constexpr char LEFT_SEPERATOR = '{';
  static constexpr char RIGHT_SEPERATOR = '}';

private:
  template <typename T>
  static void _internal_print(std::ostream &os, const T &s) {
    os << s;
  }

  static constexpr size_t find_left_bracket(const std::string_view &s,
                                            size_t idx = 0) {
    return s.find(LEFT_SEPERATOR, idx);
  }

  static constexpr size_t find_right_bracket(const std::string_view &s,
                                             size_t idx = 0) {
    return s.find(RIGHT_SEPERATOR, idx);
  }

  static constexpr std::pair<size_t, size_t>
  find_separator(const std::string_view &s, size_t idx = 0) {
    size_t i = find_left_bracket(s, idx);
    size_t j = find_right_bracket(s, i + 1);
    return std::make_pair(i, j);
  }

  static constexpr bool escaped(const std::string_view &s, size_t i, size_t j) {
    return j + 1 < s.length() && s[i + 1] == LEFT_SEPERATOR &&
           s[j + 1] == RIGHT_SEPERATOR;
  }

public:
  template <typename T, typename... Args>
  static void format(std::ostream &os, const std::string_view &s, const T &arg,
                     const Args &...args) {
    if (auto [i, j] = find_separator(s);
        i != std::string_view::npos && j != std::string_view::npos) {
      _internal_print(os, s.substr(0, i));

      if (escaped(s, i, j)) {
        _internal_print(os, s.substr(i + 1, j - i - 1));
        return format(os, s.substr(j + 1), arg, args...);
      }

      _internal_print(os, arg);
      return format(os, s.substr(j + 1), args...);
    }
    _internal_print(os, s);
  }

  template <typename T, typename... Args>
  static void format(const std::string_view &s, const T &arg,
                     const Args &...args) {
    format(std::cout, s, arg, args...);
  }

  static void format(const std::string_view &s) { format(std::cout, s); }
  static void format(std::ostream &os, const std::string_view &s) {
    size_t prev = 0;
    auto [i, j] = find_separator(s);

    while (i != std::string_view::npos && j != std::string_view::npos) {
      _internal_print(os, s.substr(prev, i - prev));

      if (escaped(s, i, j)) {
        _internal_print(os, s.substr(i + 1, j - i - 1));
      }

      prev = j + 1;
      std::pair<size_t, size_t> separator_pair{find_separator(s, j + 1)};
      i = separator_pair.first;
      j = separator_pair.second;
    }

    if (prev < s.length()) {
      _internal_print(os, s.substr(prev));
    }
  }
};