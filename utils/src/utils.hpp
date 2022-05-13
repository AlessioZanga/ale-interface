#pragma once
#include <string>

namespace utils {

// Convert `const char*` to `std::string`.
std::string std_string_from(const char* s, size_t n);

// Convert `std::string` to `const char*`.
const char* std_string_into(const std::string& s);

}  // namespace utils
