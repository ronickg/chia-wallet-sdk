#pragma once
#include "chia_wallet_ffi.h"
#include "clvm.hpp"
#include <memory>
#include <optional>
#include <array>
#include <iomanip>
#include <sstream>
#include <vector>

namespace chia
{
    class Bytes
    {
    private:
        BytesHandle *handle;

    public:
        Bytes() : handle(bytes_create())
        {
            if (!handle)
            {
                throw std::runtime_error("Failed to create bytes handle");
            }
        }

        explicit Bytes(const std::vector<uint8_t> &data)
            : handle(bytes_from_slice(data.empty() ? nullptr : data.data(), data.size()))
        {
            if (!handle)
            {
                throw std::runtime_error("Failed to create bytes from data");
            }
        }

        // Move constructor
        Bytes(Bytes &&other) noexcept : handle(other.handle)
        {
            other.handle = nullptr;
        }

        // Move assignment
        Bytes &operator=(Bytes &&other) noexcept
        {
            if (this != &other)
            {
                if (handle)
                {
                    bytes_destroy(handle);
                }
                handle = other.handle;
                other.handle = nullptr;
            }
            return *this;
        }

        // No copying
        Bytes(const Bytes &) = delete;
        Bytes &operator=(const Bytes &) = delete;

        ~Bytes()
        {
            if (handle)
            {
                bytes_destroy(handle);
            }
        }

        // Get data back as vector
        std::vector<uint8_t> to_vector() const
        {
            if (!handle)
                return std::vector<uint8_t>();

            const size_t len = bytes_len(handle);
            std::vector<uint8_t> result(len);

            if (len > 0)
            {
                if (!bytes_copy_to(handle, result.data(), len))
                {
                    throw std::runtime_error("Failed to copy bytes data");
                }
            }

            return result;
        }

        // Get length
        size_t size() const
        {
            return handle ? bytes_len(handle) : 0;
        }

        // Check if empty
        bool empty() const
        {
            return size() == 0;
        }

        // Get raw handle for FFI calls
        const BytesHandle *raw_handle() const { return handle; }
        BytesHandle *raw_handle() { return handle; }
    };

    template <typename BytesType, size_t N>
    class BytesN
    {
    public:
        // Default constructor - creates empty hash
        BytesN()
        {
            std::memset(bytes_.data, 0, N);
        }

        // Constructor from raw bytes
        explicit BytesN(const uint8_t *data)
        {
            if (data)
            {
                std::memcpy(bytes_.data, data, N);
            }
            else
            {
                std::memset(bytes_.data, 0, N);
            }
        }

        // Constructor from std::array
        explicit BytesN(const std::array<uint8_t, N> &arr)
        {
            std::memcpy(bytes_.data, arr.data(), N);
        }

        // Constructor from FFI Bytes type
        explicit BytesN(const BytesType &bytes)
        {
            std::memcpy(bytes_.data, bytes.data, N);
        }

        // Get underlying FFI Bytes type
        const BytesType &bytes() const { return bytes_; }
        BytesType &bytes() { return bytes_; }

        // Convert to std::array
        std::array<uint8_t, N> to_array() const
        {
            std::array<uint8_t, N> result;
            std::memcpy(result.data(), bytes_.data, N);
            return result;
        }

        // Comparison operators
        bool operator==(const BytesN &other) const
        {
            return std::memcmp(bytes_.data, other.bytes_.data, N) == 0;
        }

        bool operator!=(const BytesN &other) const
        {
            return !(*this == other);
        }

        // Access underlying data
        const uint8_t *data() const { return bytes_.data; }
        uint8_t *data() { return bytes_.data; }

        // Check if empty (all zeros)
        bool is_empty() const
        {
            const uint8_t zeros[N] = {0};
            return std::memcmp(bytes_.data, zeros, N) == 0;
        }

        // Get size
        static constexpr size_t size() { return N; }

    private:
        BytesType bytes_;
    };

    // Type aliases using the FFI types
    using Bytes32 = BytesN<::Bytes32, 32>;
    using Bytes48 = BytesN<::Bytes48, 48>;
    using Bytes96 = BytesN<::Bytes96, 96>;
    using Bytes100 = BytesN<::Bytes100, 100>;

} // namespace chia