#pragma once
#include "chia_wallet_ffi.h"
#include "clvm.hpp"
#include "bytes.hpp"
#include <memory>
#include <optional>
#include <array>
#include <iomanip>
#include <sstream>
#include <array>

namespace chia
{
    class Program
    {
    private:
        ProgramHandle *program;
        std::reference_wrapper<const Clvm> clvm_ref;

    public:
        Program(const Clvm &clvm_context, ProgramHandle *ptr)
            : program(ptr), clvm_ref(clvm_context)
        {
            if (!program)
            {
                throw std::runtime_error("Invalid program pointer");
            }
        }

        // Copy operations - now allowed since NodePtr in Rust is copyable
        Program(const Program &other)
            : program(other.program), clvm_ref(other.clvm_ref)
        {
        }

        Program &operator=(const Program &other)
        {
            if (this != &other)
            {
                program = other.program;
                clvm_ref = other.clvm_ref;
            }
            return *this;
        }

        // Move operations
        Program(Program &&other) noexcept
            : program(other.program), clvm_ref(other.clvm_ref)
        {
            other.program = nullptr;
        }

        Program &operator=(Program &&other) noexcept
        {
            if (this != &other)
            {
                program = other.program;
                clvm_ref = other.clvm_ref;
                other.program = nullptr;
            }
            return *this;
        }

        bool is_atom() const
        {
            return program && program_is_atom(program);
        }

        bool is_pair() const
        {
            return program && program_is_pair(program);
        }

        std::string to_string() const
        {
            if (!program)
                return "";

            char *str = program_to_string(clvm_ref.get().raw_handle(), program);
            if (!str)
                return "";

            std::string result(str);
            string_destroy(str);
            return result;
        }

        double to_number() const
        {
            if (!program)
                return std::numeric_limits<double>::quiet_NaN();

            return program_to_number(clvm_ref.get().raw_handle(), program);
        }

        Bytes32 tree_hash() const
        {
            if (!program)
                return Bytes32();

            // Get raw Bytes32 from FFI call
            auto raw_result = program_tree_hash(clvm_ref.get().raw_handle(), program);

            // Construct ChiaBytes32 explicitly from the raw result
            return Bytes32(raw_result);
        }

        std::vector<uint8_t> to_bigint_bytes() const
        {
            if (!program)
                return std::vector<uint8_t>();

            auto handle = program_to_bigint_bytes(clvm_ref.get().raw_handle(), program);
            if (!handle)
                return std::vector<uint8_t>();

            auto len = bytes_len(handle);
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

        std::vector<Program> to_list() const
        {
            std::vector<Program> result;
            if (!program || !is_pair())
            {
                return result;
            }

            // Get first element
            ProgramHandle *current = program;
            while (current && program_is_pair(current))
            {
                // Get first of pair
                ProgramHandle *first = program_first(clvm_ref.get().raw_handle(), current);
                if (first)
                {
                    result.push_back(Program(clvm_ref.get(), first));
                }

                // Move to rest of list
                current = program_rest(clvm_ref.get().raw_handle(), current);
            }

            return result;
        }

        static std::string hash_to_hex(const Bytes32 &bytes32)
        {
            std::stringstream ss;
            ss << std::hex << std::setfill('0');

            // Use bytes() to get the underlying Bytes32 struct which contains the data array
            const auto &raw_bytes = bytes32.bytes();

            for (size_t i = 0; i < 32; ++i)
            {
                ss << std::setw(2) << static_cast<int>(raw_bytes.data[i]);
            }
            return ss.str();
        }

        static Program nil(const Clvm &clvm)
        {
            auto ptr = clvm_nil_program(clvm.raw_handle());
            return Program(clvm, ptr);
        }

        ProgramHandle *raw_handle() const { return program; }
    };
} // namespace chia