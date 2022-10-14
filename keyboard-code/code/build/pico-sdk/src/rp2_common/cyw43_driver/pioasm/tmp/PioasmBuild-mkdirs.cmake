# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION 3.5)

file(MAKE_DIRECTORY
  "D:/Documents/Github/PCB-Design/keyboard-code/pico-sdk/tools/pioasm"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/pioasm"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/pico-sdk/src/rp2_common/cyw43_driver/pioasm"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/pico-sdk/src/rp2_common/cyw43_driver/pioasm/tmp"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/pico-sdk/src/rp2_common/cyw43_driver/pioasm/src/PioasmBuild-stamp"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/pico-sdk/src/rp2_common/cyw43_driver/pioasm/src"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/pico-sdk/src/rp2_common/cyw43_driver/pioasm/src/PioasmBuild-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "D:/Documents/Github/PCB-Design/keyboard-code/code/build/pico-sdk/src/rp2_common/cyw43_driver/pioasm/src/PioasmBuild-stamp/${subDir}")
endforeach()
