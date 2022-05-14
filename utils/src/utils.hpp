#pragma once

#include <string>
#include <tuple>
#include <vector>

namespace utils {

// Convert `std::vector<uint32_t>` to `uint32_t*`.
uint32_t* std_vector_u32_buffer(std::vector<uint32_t>& v);

// Get `std::vector<uint32_t>` length.
size_t std_vector_u32_length(const std::vector<uint32_t>& v);

// Get `std::vector<uint32_t>` capacity.
size_t std_vector_u32_capacity(const std::vector<uint32_t>& v);

// Convert `const char*` to `std::string`.
std::string std_string_from(const uint8_t* s, size_t n);

// Convert `std::string` to `char*`.
uint8_t* std_string_buffer(std::string& s);

// Get `std::string` length.
size_t std_string_length(const std::string& s);

// Get `std::string` capacity.
size_t std_string_capacity(const std::string& s);

}  // namespace utils
