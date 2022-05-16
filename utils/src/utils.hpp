#pragma once

#include <filesystem>
#include <string>

namespace utils {

// Convert std::string to std::filesystem::path.
std::filesystem::path string_to_path(const std::string& path);

}  // namespace utils
