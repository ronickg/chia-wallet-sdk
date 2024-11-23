#pragma once
#include "chia_wallet_ffi.h"
#include <memory>
#include <string>
#include "bytes.hpp"

namespace chia
{

    // class ClvmArray
    // {
    // private:
    //     std::unique_ptr<::ClvmValueArray> array;
    //     std::unique_ptr<::ClvmValue[]> items;

    // public:
    //     explicit ClvmArray(std::vector<ClvmValue> &values)
    //         : items(std::make_unique<::ClvmValue[]>(values.size()))
    //     {
    //         // Move values into our array
    //         for (size_t i = 0; i < values.size(); i++)
    //         {
    //             items[i] = *values[i].raw_handle(); // Copy the struct
    //             values[i].release();                // Release ownership without freeing
    //         }

    //         // Create the array struct
    //         array = std::make_unique<::ClvmValueArray>();
    //         array->items = items.get();
    //         array->len = values.size();
    //     }

    //     // No copy
    //     ClvmArray(const ClvmArray &) = delete;
    //     ClvmArray &operator=(const ClvmArray &) = delete;

    //     // Allow move
    //     ClvmArray(ClvmArray &&) = default;
    //     ClvmArray &operator=(ClvmArray &&) = default;

    //     // Get raw pointer for FFI (transfers ownership)
    //     ::ClvmValueArray *release()
    //     {
    //         items.release();        // Release ownership of items array
    //         return array.release(); // Release ownership of array struct
    //     }
    // };

    class ClvmValue
    {
    private:
        ::ClvmValue *handle;

    public:
        // Constructor for number values
        static ClvmValue createNumber(double value)
        {
            ClvmValue clvm_value;
            clvm_value.handle = new ::ClvmValue{
                .value_type = ClvmValueType::Number,
                .data = {.number = value}};
            return clvm_value;
        }

        // Constructor for string values
        static ClvmValue createString(const std::string &value)
        {
            ClvmValue clvm_value;
            char *str = new char[value.length() + 1];
            std::strcpy(str, value.c_str());
            clvm_value.handle = new ::ClvmValue{
                .value_type = ClvmValueType::String,
                .data = {.string = str}};
            return clvm_value;
        }

        // Constructor for bigint values
        static ClvmValue createBigInt(const std::vector<uint8_t> &value)
        {
            ClvmValue clvm_value;
            BytesHandle *handle = bytes_from_slice(value.empty() ? nullptr : value.data(), value.size());
            if (!handle)
            {
                throw std::runtime_error("Failed to create bytes from data");
            }
            clvm_value.handle = new ::ClvmValue{
                .value_type = ClvmValueType::BigInt,
                .data = {.bigint = handle}};
            return clvm_value;
        }

        // Then add to C++ ClvmValue class:
        // static ClvmValue createArray(std::vector<ClvmValue> &values)
        // {
        //     ClvmValue clvm_value;

        //     // Create array using our safe wrapper
        //     ClvmArray safe_array(values);

        //     // Create the ClvmValue and transfer ownership
        //     clvm_value.handle = new ::ClvmValue{
        //         .value_type = ClvmValueType::Array,
        //         .data = {.array = safe_array.release()}};

        //     return clvm_value;
        // }
        static ClvmValue createArray(std::vector<ClvmValue> &values); // Declaration only

        // Move constructor
        ClvmValue(ClvmValue &&other) noexcept : handle(other.handle)
        {
            other.handle = nullptr;
        }

        // Move assignment
        ClvmValue &operator=(ClvmValue &&other) noexcept
        {
            if (this != &other)
            {
                if (handle)
                {
                    clvm_value_free(handle);
                }
                handle = other.handle;
                other.handle = nullptr;
            }
            return *this;
        }

        // Delete copy operations
        ClvmValue(const ClvmValue &) = delete;
        ClvmValue &operator=(const ClvmValue &) = delete;

        ~ClvmValue()
        {
            if (handle)
            {
                clvm_value_free(handle);
            }
        }

        void release()
        {
            handle = nullptr;
        }

        // Get raw handle for FFI calls
        const ::ClvmValue *raw_handle() const { return handle; }
        ::ClvmValue *raw_handle() { return handle; }

    private:
        ClvmValue() : handle(nullptr) {}
    };

    // ClvmArray can be in the same header after ClvmValue
    class ClvmArray
    {
    private:
        std::unique_ptr<::ClvmValueArray> array;
        std::unique_ptr<::ClvmValue[]> items;

    public:
        explicit ClvmArray(std::vector<ClvmValue> &values)
            : items(std::make_unique<::ClvmValue[]>(values.size()))
        {
            // Move values into our array
            for (size_t i = 0; i < values.size(); i++)
            {
                items[i] = *values[i].raw_handle();
                values[i].release();
            }

            // Create the array struct
            array = std::make_unique<::ClvmValueArray>();
            array->items = items.get();
            array->len = values.size();
        }

        // No copy
        ClvmArray(const ClvmArray &) = delete;
        ClvmArray &operator=(const ClvmArray &) = delete;

        // Allow move
        ClvmArray(ClvmArray &&) = default;
        ClvmArray &operator=(ClvmArray &&) = default;

        ::ClvmValueArray *release()
        {
            items.release();
            return array.release();
        }
    };

    // Implementation of createArray (needs to be after ClvmArray definition)
    inline ClvmValue ClvmValue::createArray(std::vector<ClvmValue> &values)
    {
        ClvmValue clvm_value;
        ClvmArray safe_array(values);
        clvm_value.handle = new ::ClvmValue{
            .value_type = ClvmValueType::Array,
            .data = {.array = safe_array.release()}};
        return clvm_value;
    }

} // namespace chia