#pragma once

#include <cstddef>
#include <iostream>
#include <string_view>

class fmt {
private:
  static constexpr char LEFT_SEPERATOR = '{';
  static constexpr char RIGHT_SEPERATOR = '}';
private:
  template <typename T> static void _internal_print(const T &s) {
    std::cout << s;
  }

  static size_t find_left_bracket(const std::string_view &s, size_t idx = 0) {
    for (size_t i = idx, size = s.length(); i < size; ++i) {
      if (s[i] == LEFT_SEPERATOR) {
        return i;
      }
    }
    return s.length();
  }

  static size_t find_right_bracket(const std::string_view &s, size_t idx = 0) {
    for (size_t i = idx, size = s.length(); i < size; ++i) {
      if (s[i] == RIGHT_SEPERATOR) {
        return i;
      }
    }
    return s.length();
  }

  static bool escaped(const std::string_view &s, size_t i, size_t j) {
    return s[i + 1] == LEFT_SEPERATOR && s[j + 1] == RIGHT_SEPERATOR;
  }

public:
  template <typename T, typename... Args>
  static void format(const std::string_view &s, const T &arg,
                     const Args &...args) {
    if (size_t i = find_left_bracket(s); i != s.length()) {
      if (size_t j = find_right_bracket(s, i + 1); j != s.length()) {
        _internal_print(s.substr(0, i));

        if (escaped(s, i, j)) {
          _internal_print(s.substr(i + 1, j - i - 1));
        } else {
          _internal_print(arg);
        }

        return format(s.substr(j + 1), args...);
      }
    }
    _internal_print(s);
  }

  static void format(const std::string_view &s) { _internal_print(s); }
};