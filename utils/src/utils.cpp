#include "utils.hpp"

namespace utils {

std::vector<uint32_t> std_vector_u32_from(const uint32_t* v, size_t n, size_t c) {    
    // Initialize the vector.
    std::vector<uint32_t> out;
    // Reserve additional capacity.
    out.reserve(c);
    // Assign values.
    out.assign(v, v + n);

    return out;
}

uint32_t* std_vector_u32_buffer(std::vector<uint32_t>& v) { return v.data(); }

size_t std_vector_u32_length(const std::vector<uint32_t>& v) { return v.size(); }

size_t std_vector_u32_capacity(const std::vector<uint32_t>& v) { return v.capacity(); }

std::string std_string_from(const uint8_t* s, size_t n) { return std::string(reinterpret_cast<const char*>(s), n); }

uint8_t* std_string_buffer(std::string& s) { return reinterpret_cast<uint8_t*>(s.data()); }

size_t std_string_length(const std::string& s) { return s.length(); }

size_t std_string_capacity(const std::string& s) { return s.capacity(); }

}  // namespace utils
