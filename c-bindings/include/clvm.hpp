// clvm.hpp
#pragma once
#include "chia_wallet_ffi.h"
#include <memory>
#include <functional>

#include "clvm_value.hpp"
#include "forward.hpp"

namespace chia
{
    class Clvm
    {
    public:
        Clvm();
        ~Clvm();

        // Prevent copying
        Clvm(const Clvm &) = delete;
        Clvm &operator=(const Clvm &) = delete;

        // Allow moving
        Clvm(Clvm &&) noexcept = default;
        Clvm &operator=(Clvm &&) noexcept = default;

        // Set cleanup callback
        void setCleanupCallback(std::function<void()> callback);

        // Allocate a CLVM value
        Program allocate(const chia::ClvmValue &value) const;

        // Get raw handle (needed for FFI calls)
        ClvmHandle *raw_handle() const;

    private:
        struct Impl
        {
            ClvmHandle *handle;
            std::function<void()> cleanup;

            Impl() : handle(clvm_create()), cleanup(nullptr)
            {
                if (!handle)
                {
                    throw std::runtime_error("Failed to create CLVM handle");
                }
            }

            ~Impl()
            {
                if (handle)
                {
                    if (cleanup)
                    {
                        cleanup();
                    }
                    clvm_destroy(handle);
                }
            }
        };

        std::unique_ptr<Impl> pImpl;
    };
} // namespace chia