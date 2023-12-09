# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.25

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/cdjk/esp/esp-idf/components/bootloader/subproject

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/cdjk/github/esp-items/esp32-80211-tx/bootloader

# Utility rule file for menuconfig.

# Include any custom commands dependencies for this target.
include CMakeFiles/menuconfig.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/menuconfig.dir/progress.make

CMakeFiles/menuconfig:
	python /home/cdjk/esp/esp-idf/tools/kconfig_new/prepare_kconfig_files.py --list-separator=semicolon --env-file /home/cdjk/github/esp-items/esp32-80211-tx/bootloader/config.env
	python -m kconfgen --list-separator=semicolon --kconfig /home/cdjk/esp/esp-idf/Kconfig --sdkconfig-rename /home/cdjk/esp/esp-idf/sdkconfig.rename --config /home/cdjk/github/esp-items/esp32-80211-tx/sdkconfig --env-file /home/cdjk/github/esp-items/esp32-80211-tx/bootloader/config.env --env IDF_TARGET=esp32 --env IDF_TOOLCHAIN=gcc --env IDF_ENV_FPGA= --env IDF_INIT_VERSION=5.3.0 --dont-write-deprecated --output config /home/cdjk/github/esp-items/esp32-80211-tx/sdkconfig
	python /home/cdjk/esp/esp-idf/tools/check_term.py
	/usr/bin/cmake -E env COMPONENT_KCONFIGS_SOURCE_FILE=/home/cdjk/github/esp-items/esp32-80211-tx/bootloader/kconfigs.in COMPONENT_KCONFIGS_PROJBUILD_SOURCE_FILE=/home/cdjk/github/esp-items/esp32-80211-tx/bootloader/kconfigs_projbuild.in KCONFIG_CONFIG=/home/cdjk/github/esp-items/esp32-80211-tx/sdkconfig IDF_TARGET=esp32 IDF_TOOLCHAIN=gcc IDF_ENV_FPGA= IDF_INIT_VERSION=5.3.0 python -m menuconfig /home/cdjk/esp/esp-idf/Kconfig
	python -m kconfgen --list-separator=semicolon --kconfig /home/cdjk/esp/esp-idf/Kconfig --sdkconfig-rename /home/cdjk/esp/esp-idf/sdkconfig.rename --config /home/cdjk/github/esp-items/esp32-80211-tx/sdkconfig --env-file /home/cdjk/github/esp-items/esp32-80211-tx/bootloader/config.env --env IDF_TARGET=esp32 --env IDF_TOOLCHAIN=gcc --env IDF_ENV_FPGA= --env IDF_INIT_VERSION=5.3.0 --output config /home/cdjk/github/esp-items/esp32-80211-tx/sdkconfig

menuconfig: CMakeFiles/menuconfig
menuconfig: CMakeFiles/menuconfig.dir/build.make
.PHONY : menuconfig

# Rule to build all files generated by this target.
CMakeFiles/menuconfig.dir/build: menuconfig
.PHONY : CMakeFiles/menuconfig.dir/build

CMakeFiles/menuconfig.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/menuconfig.dir/cmake_clean.cmake
.PHONY : CMakeFiles/menuconfig.dir/clean

CMakeFiles/menuconfig.dir/depend:
	cd /home/cdjk/github/esp-items/esp32-80211-tx/bootloader && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/cdjk/esp/esp-idf/components/bootloader/subproject /home/cdjk/esp/esp-idf/components/bootloader/subproject /home/cdjk/github/esp-items/esp32-80211-tx/bootloader /home/cdjk/github/esp-items/esp32-80211-tx/bootloader /home/cdjk/github/esp-items/esp32-80211-tx/bootloader/CMakeFiles/menuconfig.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/menuconfig.dir/depend

