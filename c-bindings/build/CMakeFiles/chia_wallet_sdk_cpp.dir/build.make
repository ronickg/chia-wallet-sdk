# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.30

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
CMAKE_COMMAND = /opt/homebrew/Cellar/cmake/3.30.5/bin/cmake

# The command to remove a file.
RM = /opt/homebrew/Cellar/cmake/3.30.5/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings/build

# Include any dependencies generated for this target.
include CMakeFiles/chia_wallet_sdk_cpp.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/chia_wallet_sdk_cpp.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/chia_wallet_sdk_cpp.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/chia_wallet_sdk_cpp.dir/flags.make

CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o: CMakeFiles/chia_wallet_sdk_cpp.dir/flags.make
CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o: /Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc
CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o: CMakeFiles/chia_wallet_sdk_cpp.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/ronaldgoedeke/chia-wallet-sdk/c-bindings/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o -MF CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o.d -o CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o -c /Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc

CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc > CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.i

CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc -o CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.s

# Object files for target chia_wallet_sdk_cpp
chia_wallet_sdk_cpp_OBJECTS = \
"CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o"

# External object files for target chia_wallet_sdk_cpp
chia_wallet_sdk_cpp_EXTERNAL_OBJECTS =

libchia_wallet_sdk_cpp.a: CMakeFiles/chia_wallet_sdk_cpp.dir/Users/ronaldgoedeke/chia-wallet-sdk/target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc.o
libchia_wallet_sdk_cpp.a: CMakeFiles/chia_wallet_sdk_cpp.dir/build.make
libchia_wallet_sdk_cpp.a: CMakeFiles/chia_wallet_sdk_cpp.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/Users/ronaldgoedeke/chia-wallet-sdk/c-bindings/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX static library libchia_wallet_sdk_cpp.a"
	$(CMAKE_COMMAND) -P CMakeFiles/chia_wallet_sdk_cpp.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/chia_wallet_sdk_cpp.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/chia_wallet_sdk_cpp.dir/build: libchia_wallet_sdk_cpp.a
.PHONY : CMakeFiles/chia_wallet_sdk_cpp.dir/build

CMakeFiles/chia_wallet_sdk_cpp.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/chia_wallet_sdk_cpp.dir/cmake_clean.cmake
.PHONY : CMakeFiles/chia_wallet_sdk_cpp.dir/clean

CMakeFiles/chia_wallet_sdk_cpp.dir/depend:
	cd /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings/build /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings/build /Users/ronaldgoedeke/chia-wallet-sdk/c-bindings/build/CMakeFiles/chia_wallet_sdk_cpp.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/chia_wallet_sdk_cpp.dir/depend
