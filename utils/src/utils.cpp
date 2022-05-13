#include "utils.hpp"

std::string std_string_from(const char* s, size_t n) {
    return std::string(s, n);
}

const char* std_string_into(const std::string &s) {
    return s.c_str();
}
