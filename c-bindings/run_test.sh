#!/bin/bash
set -e  # Exit on any error

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "Project root: ${PROJECT_ROOT}"

# Build Rust library
echo "Building Rust library..."
cd "${PROJECT_ROOT}"
cargo build --release -p chia-wallet-sdk-c-bindings

# Create include directory if it doesn't exist
mkdir -p "${PROJECT_ROOT}/c-bindings/include"

# Generate header using cbindgen
echo "Generating C header..."
cbindgen \
    --config "${PROJECT_ROOT}/c-bindings/cbindgen.toml" \
    --crate chia-wallet-sdk-c-bindings \
    --output "${PROJECT_ROOT}/c-bindings/include/chia_wallet_ffi.h" \
    "${PROJECT_ROOT}/c-bindings"

# Verify header was generated
if [ ! -f "${PROJECT_ROOT}/c-bindings/include/chia_wallet_ffi.h" ]; then
    echo "Error: Header file was not generated!"
    exit 1
fi

# Print first few lines of header for verification
echo "Generated header preview:"
head -n 20 "${PROJECT_ROOT}/c-bindings/include/chia_wallet_ffi.h"

# Setup build directory in c-bindings
echo "Setting up CMake build..."
cd "${PROJECT_ROOT}/c-bindings"
mkdir -p build
cd build

# Configure and build with CMake
echo "Configuring and building tests..."
cmake ..
cmake --build .

# Run tests
echo "Running tests..."
./test_clvm

echo "Done!"
# # #!/bin/bash
# # set -e  # Exit on any error

# # # Get the project root directory
# # PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# # echo "Project root: ${PROJECT_ROOT}"

# # # Build Rust library
# # echo "Building Rust library..."
# # cd "${PROJECT_ROOT}"
# # cargo build --release -p chia-wallet-sdk-c-bindings

# # # Create include directory if it doesn't exist
# # mkdir -p "${PROJECT_ROOT}/c-bindings/include"

# # # Generate header using cbindgen
# # # echo "Generating C header..."
# # # cbindgen \
# # #     --config "${PROJECT_ROOT}/c-bindings/cbindgen.toml" \
# # #     --crate chia-wallet-sdk-c-bindings \
# # #     --output "${PROJECT_ROOT}/c-bindings/include/chia_wallet_ffi.h" \
# # #     "${PROJECT_ROOT}/c-bindings"

# # # # Verify header was generated
# # # if [ ! -f "${PROJECT_ROOT}/c-bindings/include/chia_wallet_ffi.h" ]; then
# # #     echo "Error: Header file was not generated!"
# # #     exit 1
# # # fi

# # # Print first few lines of header for verification
# # echo "Generated header preview:"
# # head -n 20 "${PROJECT_ROOT}/c-bindings/include/chia_wallet_ffi.h"

# # # Setup build directory in c-bindings
# # echo "Setting up CMake build..."
# # cd "${PROJECT_ROOT}/c-bindings"
# # mkdir -p build
# # cd build

# # # Configure and build with CMake
# # echo "Configuring and building tests..."
# # cmake ..
# # cmake --build .

# # # Run tests
# # echo "Running tests..."
# # ./test_clvm

# # echo "Done!"
# #!/bin/bash
# set -e

# PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# # Build Rust library
# cd "${PROJECT_ROOT}"
# cargo build --release -p chia-wallet-sdk-c-bindings


# cd "${PROJECT_ROOT}/cpp"
# mkdir -p build && cd build
# cmake ..
# cmake --build .
# # #!/bin/bash
# # set -e  # Exit on any error

# # # Build Rust library
# # echo "Building Rust library..."
# # cargo build --release

# # # Setup build directory in c-bindings
# # echo "Setting up CMake build..."
# # mkdir -p build
# # cd build

# # # Configure and build with CMake
# # echo "Configuring and building tests..."
# # cmake ..
# # cmake --build .

# # Run tests
# echo "Running tests..."
# ./test_clvm

# echo "Done!"
