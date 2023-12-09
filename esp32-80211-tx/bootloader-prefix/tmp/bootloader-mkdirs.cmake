# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION 3.5)

file(MAKE_DIRECTORY
  "/home/cdjk/esp/esp-idf/components/bootloader/subproject"
  "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader"
  "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader-prefix"
  "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader-prefix/tmp"
  "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader-prefix/src/bootloader-stamp"
  "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader-prefix/src"
  "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader-prefix/src/bootloader-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader-prefix/src/bootloader-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "/home/cdjk/github/esp-items/esp32-80211-tx/bootloader-prefix/src/bootloader-stamp${cfgdir}") # cfgdir has leading slash
endif()
