cmake_minimum_required(VERSION 3.13)

if(MSVC)
    add_definitions(-D_CRT_SECURE_NO_WARNINGS)
endif()

# include pico-sdk next to the workspace folder
include(../pico-sdk/external/pico_sdk_import.cmake)
project(orionshands C CXX ASM)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

pico_sdk_init()

add_executable(${PROJECT_NAME}
  src/main.c
)
target_link_libraries(${PROJECT_NAME} 
  pico_stdlib
)

# create map/bin/hex file etc.
pico_add_extra_outputs(${PROJECT_NAME})