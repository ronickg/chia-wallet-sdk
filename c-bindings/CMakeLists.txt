cmake_minimum_required(VERSION 3.14)
project(chia_bindings_tests)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add path to Rust library
set(RUST_TARGET_DIR ${CMAKE_SOURCE_DIR}/../target/release)

# Platform-specific library name
if(APPLE)
    set(RUST_LIB_NAME "${RUST_TARGET_DIR}/libchia_wallet_sdk_c_bindings.dylib")
elseif(UNIX)
    set(RUST_LIB_NAME "${RUST_TARGET_DIR}/libchia_wallet_sdk_c_bindings.so")
elseif(WIN32)
    set(RUST_LIB_NAME "${RUST_TARGET_DIR}/chia_wallet_sdk_c_bindings.dll")
endif()

# Find GTest package
find_package(GTest REQUIRED)

# Add test executable with all source files
add_executable(test_clvm
    tests/utils.cpp
    src/clvm.cpp    # Add this line to include the implementation
)

# Add include directories
target_include_directories(test_clvm
    PRIVATE
    ${CMAKE_SOURCE_DIR}/include
    ${GTEST_INCLUDE_DIRS}
)

# Link libraries
target_link_libraries(test_clvm
    PRIVATE
    ${RUST_LIB_NAME}  # Use the platform-specific library path
    GTest::GTest
    GTest::Main
)

# On macOS, we might need to modify the runtime path
if(APPLE)
    add_custom_command(TARGET test_clvm POST_BUILD
        COMMAND ${CMAKE_INSTALL_NAME_TOOL} -add_rpath "@loader_path/../../../target/release"
        $<TARGET_FILE:test_clvm>
    )
endif()

# Enable testing
enable_testing()
add_test(NAME test_clvm COMMAND test_clvm)

# Optional: Add sanitizer build
option(USE_SANITIZER "Build with Address Sanitizer" OFF)
if(USE_SANITIZER)
    target_compile_options(test_clvm PRIVATE -fsanitize=address -fno-omit-frame-pointer)
    target_link_options(test_clvm PRIVATE -fsanitize=address)
endif()

# Print debug information
message(STATUS "Rust library path: ${RUST_LIB_NAME}")
message(STATUS "Include directories: ${CMAKE_SOURCE_DIR}/include")