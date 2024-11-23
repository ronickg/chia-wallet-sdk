# cmake_minimum_required(VERSION 3.14)
# project(chia_bindings_tests)

# set(CMAKE_CXX_STANDARD 17)
# set(CMAKE_CXX_STANDARD_REQUIRED ON)

# # Specify Rust library path and name
# link_directories(${CMAKE_SOURCE_DIR}/../target/release)

# add_library(chia_wallet_sdk_cpp
#     ${CMAKE_SOURCE_DIR}/../target/cxxbridge/chia-wallet-sdk-c-bindings/src/lib.rs.cc
# )

# message("CXX Bindings Path: ${CMAKE_SOURCE_DIR}/../target/cxxbridge/chia-wallet-sdk-c-bindings/src")
# message("Rust Path: ${CMAKE_SOURCE_DIR}/../target/cxxbridge/rust")


# target_include_directories(chia_wallet_sdk_cpp PUBLIC
#     ${CMAKE_SOURCE_DIR}/../target/cxxbridge/chia-wallet-sdk-c-bindings/src
#     ${CMAKE_SOURCE_DIR}/../target/cxxbridge/rust
# )

# find_package(GTest REQUIRED)

# add_executable(test_clvm tests/utils.cpp)

# target_link_libraries(test_clvm PRIVATE
#     chia_wallet_sdk_cpp
#     ${CMAKE_SOURCE_DIR}/../target/release/libchia_wallet_sdk_cpp_bindings.a
#     GTest::GTest
#     GTest::Main
# )

# enable_testing()
# add_test(NAME test_clvm COMMAND test_clvm)
cmake_minimum_required(VERSION 3.14)
project(chia_bindings_tests)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add path to Rust library
set(RUST_TARGET_DIR ${CMAKE_SOURCE_DIR}/../target/release)
link_directories(${RUST_TARGET_DIR})

# Find GTest package
find_package(GTest REQUIRED)

# Platform-specific library name
if(APPLE)
    set(RUST_LIB_NAME "${RUST_TARGET_DIR}/libchia_wallet_sdk_c_bindings.dylib")
elseif(UNIX)
    set(RUST_LIB_NAME "${RUST_TARGET_DIR}/libchia_wallet_sdk_c_bindings.so")
elseif(WIN32)
    set(RUST_LIB_NAME "${RUST_TARGET_DIR}/chia_wallet_sdk_c_bindings.dll")
endif()

# Add test executable
add_executable(test_clvm
    tests/utils.cpp
    src/clvm.cpp
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

# Enable testing
enable_testing()
add_test(NAME test_clvm COMMAND test_clvm)

# Optional: Add sanitizer build
option(USE_SANITIZER "Build with Address Sanitizer" OFF)
if(USE_SANITIZER)
    target_compile_options(test_clvm PRIVATE -fsanitize=address -fno-omit-frame-pointer)
    target_link_options(test_clvm PRIVATE -fsanitize=address)
endif()
# cmake_minimum_required(VERSION 3.14)
# project(chia_bindings_tests)

# # Set C++ standard
# set(CMAKE_CXX_STANDARD 17)
# set(CMAKE_CXX_STANDARD_REQUIRED ON)

# # Add path to Rust library
# link_directories(${CMAKE_SOURCE_DIR}/../target/release)

# # Find GTest package
# find_package(GTest REQUIRED)

# # Add test executable
# add_executable(test_clvm
#     tests/utils.cpp
#     # src/ChiaWallet.cpp  # Add the new source file
# )

# # Add include directories
# target_include_directories(test_clvm
#     PRIVATE
#     ${CMAKE_SOURCE_DIR}/include
#     ${GTEST_INCLUDE_DIRS}
# )

# # Link libraries
# target_link_libraries(test_clvm
#     PRIVATE
#     chia_wallet_sdk_c_bindings
#     GTest::GTest
#     GTest::Main
# )

# # Enable testing
# enable_testing()
# add_test(NAME test_clvm COMMAND test_clvm)

# # Optional: Add sanitizer build
# option(USE_SANITIZER "Build with Address Sanitizer" OFF)
# if(USE_SANITIZER)
#     target_compile_options(test_clvm PRIVATE -fsanitize=address -fno-omit-frame-pointer)
#     target_link_options(test_clvm PRIVATE -fsanitize=address)
# endif()

# cmake_minimum_required(VERSION 3.14)
# project(chia_bindings_tests)

# # Set C++ standard
# set(CMAKE_CXX_STANDARD 17)
# set(CMAKE_CXX_STANDARD_REQUIRED ON)

# # Add path to Rust library
# link_directories(${CMAKE_SOURCE_DIR}/../target/cxxbridge)

# # Find GTest package
# find_package(GTest REQUIRED)

# # Add test executable
# add_executable(test_clvm
#     tests/utils.cpp
#     # src/ChiaWallet.cpp  # Add the new source file
# )

# # Add include directories
# target_include_directories(test_clvm
#     PRIVATE
#     ${CMAKE_SOURCE_DIR}/include
#     ${GTEST_INCLUDE_DIRS}
# )

# # Link libraries
# target_link_libraries(test_clvm
#     PRIVATE
#     chia_wallet_sdk_c_bindings
#     GTest::GTest
#     GTest::Main
# )

# # Enable testing
# enable_testing()
# add_test(NAME test_clvm COMMAND test_clvm)

# # Optional: Add sanitizer build
# option(USE_SANITIZER "Build with Address Sanitizer" OFF)
# if(USE_SANITIZER)
#     target_compile_options(test_clvm PRIVATE -fsanitize=address -fno-omit-frame-pointer)
#     target_link_options(test_clvm PRIVATE -fsanitize=address)
# endif()
