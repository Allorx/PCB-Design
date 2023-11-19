cmake_minimum_required(VERSION 3.13)

# windblows
add_definitions(-D_CRT_SECURE_NO_WARNINGS)

# include pico-sdk next to the workspace folder
include(../pico-sdk/external/pico_sdk_import.cmake)
project(orionshands C CXX ASM)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# set board to pico w
set(PICO_BOARD pico_w)

pico_sdk_init()

add_executable(${PROJECT_NAME}
  src/main.c
)
target_link_libraries(${PROJECT_NAME} 
  pico_stdlib
  pico_btstack_classic
  pico_btstack_cyw43
  pico_cyw43_arch_none
  pico_multicore
)

target_include_directories(${PROJECT_NAME} PUBLIC
 src
)

# create map/bin/hex file etc.
pico_add_extra_outputs(${PROJECT_NAME})

# enable usb or uart output
pico_enable_stdio_usb(${PROJECT_NAME} 0)
pico_enable_stdio_uart(${PROJECT_NAME} 0)
