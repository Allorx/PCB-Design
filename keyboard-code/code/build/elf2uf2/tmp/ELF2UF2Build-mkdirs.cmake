# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION 3.5)

file(MAKE_DIRECTORY
  "D:/Documents/Github/PCB-Design/keyboard-code/pico-sdk/tools/elf2uf2"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/elf2uf2"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/elf2uf2"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/elf2uf2/tmp"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/elf2uf2/src/ELF2UF2Build-stamp"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/elf2uf2/src"
  "D:/Documents/Github/PCB-Design/keyboard-code/code/build/elf2uf2/src/ELF2UF2Build-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "D:/Documents/Github/PCB-Design/keyboard-code/code/build/elf2uf2/src/ELF2UF2Build-stamp/${subDir}")
endforeach()
