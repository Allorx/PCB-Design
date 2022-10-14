cmake_minimum_required(VERSION 3.13)
include(pico_sdk_import.cmake)
project(orionshands C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
pico_sdk_init()
add_executable(orionshands
  src/main.c
)
target_link_libraries(orionshands pico_stdlib)

# create map/bin/hex file etc.
pico_add_extra_outputs(orionshands)