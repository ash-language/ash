cmake_minimum_required(VERSION 3.10)

set(CMAKE_C_COMPILER clang)
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(ash C)

file(GLOB_RECURSE SRC_FILES "src/*.c")
add_executable(ash ${SRC_FILES})

target_include_directories(ash PRIVATE src)
target_compile_options(ash PRIVATE 
    -Wall 
    -Wextra
    -g
)

set_target_properties(ash PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/build
)