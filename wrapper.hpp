// Include STD types.
#include <string>
// Include the ALE interface.
#include "ale/src/ale_interface.hpp"

// Convert `const char*` to `std::string`.
std::string std_string_from(const char* s, size_t n) {
    return std::string(s, n);
}

// Convert `std::string` to `const char*`.
const char* std_string_into(const std::string &s) {
    return s.c_str();
}
