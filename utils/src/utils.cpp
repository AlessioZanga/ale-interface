#include "utils.hpp"

namespace utils {

std::filesystem::path string_to_path(const std::string& path) { return std::filesystem::path(path); }

}  // namespace utils
