cmake_minimum_required (VERSION 3.14)

project(utils VERSION 0.0.1
        DESCRIPTION "C++ utils for Rust FFI."
        LANGUAGES CXX)

set(CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})

add_subdirectory(src)
